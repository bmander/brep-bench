//! An RGB image as a PNG file: eight bits a channel, no filter, compressed as hard as zlib can.

use flate2::{Compression, Crc, write::ZlibEncoder};
use std::io::Write;

fn chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    out.extend((data.len() as u32).to_be_bytes());
    out.extend(kind);
    out.extend(data);
    let mut crc = Crc::new();
    crc.update(kind);
    crc.update(data);
    out.extend(crc.sum().to_be_bytes());
}

/// The PNG file of an image `width` pixels across, its pixels `rgb` in rows from the top.
pub fn encode(width: usize, height: usize, rgb: &[u8]) -> Vec<u8> {
    assert_eq!(rgb.len(), width * height * 3);
    // each row behind the byte that says it is not filtered
    let mut zlib = ZlibEncoder::new(Vec::new(), Compression::best());
    for row in rgb.chunks_exact(width * 3) {
        zlib.write_all(&[0]).unwrap();
        zlib.write_all(row).unwrap();
    }
    let mut header = Vec::with_capacity(13);
    header.extend((width as u32).to_be_bytes());
    header.extend((height as u32).to_be_bytes());
    // eight bits a channel, RGB, and the only compression, filtering and (no) interlacing
    header.extend([8, 2, 0, 0, 0]);
    let mut out = b"\x89PNG\r\n\x1a\n".to_vec();
    chunk(&mut out, b"IHDR", &header);
    chunk(&mut out, b"IDAT", &zlib.finish().unwrap());
    chunk(&mut out, b"IEND", &[]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::read::ZlibDecoder;
    use std::io::Read;

    /// The file's chunks, each checked against its checksum: (kind, data).
    fn chunks(file: &[u8]) -> Vec<(String, Vec<u8>)> {
        assert_eq!(&file[..8], b"\x89PNG\r\n\x1a\n");
        let mut at = 8;
        let mut out = Vec::new();
        while at < file.len() {
            let len = u32::from_be_bytes(file[at..at + 4].try_into().unwrap()) as usize;
            let body = &file[at + 4..at + 8 + len];
            let mut crc = Crc::new();
            crc.update(body);
            let sum = u32::from_be_bytes(file[at + 8 + len..at + 12 + len].try_into().unwrap());
            assert_eq!(crc.sum(), sum);
            out.push((
                String::from_utf8(body[..4].to_vec()).unwrap(),
                body[4..].to_vec(),
            ));
            at += 12 + len;
        }
        out
    }

    #[test]
    fn an_image_is_read_back() {
        let (width, height) = (3, 2);
        let rgb: Vec<u8> = (0..width * height * 3).map(|i| (i * 13) as u8).collect();
        let got = chunks(&encode(width, height, &rgb));
        let kinds: Vec<&str> = got.iter().map(|(k, _)| k.as_str()).collect();
        assert_eq!(kinds, ["IHDR", "IDAT", "IEND"]);
        assert_eq!(got[0].1, [0, 0, 0, 3, 0, 0, 0, 2, 8, 2, 0, 0, 0]);
        let mut raw = Vec::new();
        ZlibDecoder::new(&got[1].1[..])
            .read_to_end(&mut raw)
            .unwrap();
        let rows: Vec<u8> = raw
            .chunks_exact(1 + width * 3)
            .flat_map(|row| {
                assert_eq!(row[0], 0);
                row[1..].to_vec()
            })
            .collect();
        assert_eq!(rows, rgb);
    }
}
