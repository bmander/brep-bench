//! As much of JSON as a request is made of: read into a `Value`, and a string written back.

use std::collections::BTreeMap;

/// A JSON value. A string is kept as bytes: Python writes an environment's bytes that are not
/// UTF-8 as lone surrogates (U+DC80 to U+DCFF, its `surrogateescape`), which are read back as the
/// bytes they stand for.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(Vec<u8>),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

impl Value {
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Object(fields) => fields.get(key),
            _ => None,
        }
    }

    pub fn bytes(&self) -> Option<&[u8]> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn number(&self) -> Option<f64> {
        match self {
            Value::Number(x) => Some(*x),
            _ => None,
        }
    }
}

/// The value `text` holds, or what is wrong with it.
pub fn parse(text: &str) -> Result<Value, String> {
    let mut p = Parser {
        text: text.as_bytes(),
        at: 0,
    };
    let value = p.value()?;
    p.space();
    if p.at != p.text.len() {
        return Err(p.wrong("something after the value"));
    }
    Ok(value)
}

struct Parser<'a> {
    text: &'a [u8],
    at: usize,
}

impl Parser<'_> {
    fn wrong(&self, what: &str) -> String {
        format!("not JSON: {what} at byte {}", self.at)
    }

    fn space(&mut self) {
        while matches!(self.text.get(self.at), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.at += 1;
        }
    }

    fn word(&mut self, word: &str, value: Value) -> Result<Value, String> {
        if self.text[self.at..].starts_with(word.as_bytes()) {
            self.at += word.len();
            Ok(value)
        } else {
            Err(self.wrong("an unknown word"))
        }
    }

    fn value(&mut self) -> Result<Value, String> {
        self.space();
        match self.text.get(self.at) {
            None => Err(self.wrong("nothing")),
            Some(b'n') => self.word("null", Value::Null),
            Some(b't') => self.word("true", Value::Bool(true)),
            Some(b'f') => self.word("false", Value::Bool(false)),
            Some(b'"') => self.string().map(Value::String),
            Some(b'[') => {
                self.at += 1;
                let mut items = Vec::new();
                loop {
                    self.space();
                    if items.is_empty() && self.text.get(self.at) == Some(&b']') {
                        break;
                    }
                    items.push(self.value()?);
                    self.space();
                    match self.text.get(self.at) {
                        Some(b',') => self.at += 1,
                        Some(b']') => break,
                        _ => return Err(self.wrong("no `,` or `]`")),
                    }
                }
                self.at += 1;
                Ok(Value::Array(items))
            }
            Some(b'{') => {
                self.at += 1;
                let mut fields = BTreeMap::new();
                loop {
                    self.space();
                    if fields.is_empty() && self.text.get(self.at) == Some(&b'}') {
                        break;
                    }
                    if self.text.get(self.at) != Some(&b'"') {
                        return Err(self.wrong("no field name"));
                    }
                    let name = String::from_utf8_lossy(&self.string()?).into_owned();
                    self.space();
                    if self.text.get(self.at) != Some(&b':') {
                        return Err(self.wrong("no `:`"));
                    }
                    self.at += 1;
                    fields.insert(name, self.value()?);
                    self.space();
                    match self.text.get(self.at) {
                        Some(b',') => self.at += 1,
                        Some(b'}') => break,
                        _ => return Err(self.wrong("no `,` or `}`")),
                    }
                }
                self.at += 1;
                Ok(Value::Object(fields))
            }
            Some(_) => {
                let from = self.at;
                while matches!(
                    self.text.get(self.at),
                    Some(b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E')
                ) {
                    self.at += 1;
                }
                std::str::from_utf8(&self.text[from..self.at])
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .map(Value::Number)
                    .ok_or_else(|| self.wrong("no value"))
            }
        }
    }

    /// The four hexadecimal digits of a `\u` escape.
    fn hex(&mut self) -> Result<u32, String> {
        let digits = self
            .text
            .get(self.at..self.at + 4)
            .and_then(|d| std::str::from_utf8(d).ok());
        let unit = digits.and_then(|d| u32::from_str_radix(d, 16).ok());
        self.at += 4;
        unit.ok_or_else(|| self.wrong("a `\\u` without four digits"))
    }

    fn string(&mut self) -> Result<Vec<u8>, String> {
        self.at += 1; // the opening quote
        let mut out = Vec::new();
        loop {
            let Some(&c) = self.text.get(self.at) else {
                return Err(self.wrong("a string without its end"));
            };
            self.at += 1;
            match c {
                b'"' => return Ok(out),
                b'\\' => {
                    let Some(&e) = self.text.get(self.at) else {
                        return Err(self.wrong("a string without its end"));
                    };
                    self.at += 1;
                    match e {
                        b'"' | b'\\' | b'/' => out.push(e),
                        b'b' => out.push(8),
                        b'f' => out.push(12),
                        b'n' => out.push(b'\n'),
                        b'r' => out.push(b'\r'),
                        b't' => out.push(b'\t'),
                        b'u' => {
                            let unit = self.hex()?;
                            let point = match unit {
                                // a pair of surrogates is one character
                                0xD800..=0xDBFF if self.text[self.at..].starts_with(b"\\u") => {
                                    let back = self.at;
                                    self.at += 2;
                                    match self.hex()? {
                                        low @ 0xDC00..=0xDFFF => {
                                            0x10000 + ((unit - 0xD800) << 10) + (low - 0xDC00)
                                        }
                                        _ => {
                                            self.at = back;
                                            unit
                                        }
                                    }
                                }
                                _ => unit,
                            };
                            match char::from_u32(point) {
                                Some(ch) => out.extend(ch.encode_utf8(&mut [0; 4]).as_bytes()),
                                // a byte that was not UTF-8, as Python escapes it
                                None if (0xDC80..=0xDCFF).contains(&point) => {
                                    out.push((point - 0xDC00) as u8)
                                }
                                None => out.extend("\u{FFFD}".as_bytes()),
                            }
                        }
                        _ => return Err(self.wrong("an unknown escape")),
                    }
                }
                _ => out.push(c),
            }
        }
    }
}

/// `text` as a JSON string, quoted.
pub fn quoted(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_request() {
        let v = parse(r#"{"argv": ["a b", "c\n"], "timeout": 1.5e1, "core": null, "env": {}}"#)
            .unwrap();
        assert_eq!(v.get("timeout").and_then(Value::number), Some(15.0));
        assert_eq!(v.get("core"), Some(&Value::Null));
        let Some(Value::Array(argv)) = v.get("argv") else {
            panic!()
        };
        assert_eq!(argv[1].bytes(), Some(&b"c\n"[..]));
        assert_eq!(v.get("env"), Some(&Value::Object(BTreeMap::new())));
    }

    #[test]
    fn reads_escapes_as_python_writes_them() {
        // é, a pair of surrogates (U+1F600), and a byte that was not UTF-8 (0xFF)
        let v = parse(r#""é😀\udcff""#).unwrap();
        assert_eq!(
            v.bytes(),
            Some(&[0xC3, 0xA9, 0xF0, 0x9F, 0x98, 0x80, 0xFF][..])
        );
    }

    #[test]
    fn refuses_what_is_not_json() {
        for text in ["", "{", "[1,]", "{\"a\" 1}", "nul", "\"a", "1 2"] {
            assert!(parse(text).is_err(), "{text}");
        }
    }

    #[test]
    fn quotes_a_string() {
        assert_eq!(quoted("a\"b\\c\n\u{1}"), r#""a\"b\\c\n\u0001""#);
    }
}
