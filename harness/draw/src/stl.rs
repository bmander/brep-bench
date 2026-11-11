//! Reading the triangles of an STL file, binary or ASCII, as harness/stl.py reads them: the same
//! file is the same triangles to both.

/// A point: x, y, z.
pub type Point = [f64; 3];
/// A triangle: its three corners, anticlockwise seen from outside the solid.
pub type Triangle = [Point; 3];

/// A binary STL's header and its count of triangles, before the triangles of fifty bytes each.
const HEADER: usize = 84;
const RECORD: usize = 50;

/// How many triangles a binary STL says it holds, where the file is exactly as long as that
/// many: else it is not a binary STL, and is read as an ASCII one.
fn binary_count(data: &[u8]) -> Option<usize> {
    let count = data.get(80..HEADER)?;
    let n = u32::from_le_bytes(count.try_into().unwrap()) as usize;
    (data.len() == HEADER + RECORD * n).then_some(n)
}

/// Whether an STL file holds a triangle at all.
pub fn has_triangles(data: &[u8]) -> bool {
    match binary_count(data) {
        Some(n) => n > 0,
        None => data.windows(6).any(|w| w == b"vertex"),
    }
}

/// What separates an ASCII STL's words: the ASCII characters Python's `str.split()` splits at.
fn is_space(b: &u8) -> bool {
    matches!(b, b'\t' | b'\n' | 0x0b | 0x0c | b'\r' | 0x1c..=0x1f | b' ')
}

/// The triangles of an STL file.
pub fn read(data: &[u8]) -> Result<Vec<Triangle>, String> {
    if let Some(n) = binary_count(data) {
        let mut tris = Vec::with_capacity(n);
        for record in data[HEADER..].chunks_exact(RECORD) {
            // the facet's normal, then its three corners, each three little-endian floats
            let mut t = [[0.0; 3]; 3];
            for (i, x) in t.iter_mut().flatten().enumerate() {
                let at = 12 + 4 * i;
                *x = f32::from_le_bytes(record[at..at + 4].try_into().unwrap()) as f64;
            }
            tris.push(t);
        }
        return Ok(tris);
    }
    let words: Vec<&[u8]> = data.split(is_space).filter(|w| !w.is_empty()).collect();
    let mut verts: Vec<Point> = Vec::new();
    for (i, word) in words.iter().enumerate() {
        if *word != b"vertex" {
            continue;
        }
        let mut p = [0.0; 3];
        for (j, x) in p.iter_mut().enumerate() {
            let word = words
                .get(i + 1 + j)
                .ok_or("an ASCII STL that ends within a vertex")?;
            *x = std::str::from_utf8(word)
                .ok()
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| {
                    format!(
                        "an ASCII STL with a vertex at {:?}, which is no number",
                        String::from_utf8_lossy(word)
                    )
                })?;
        }
        verts.push(p);
    }
    if !verts.len().is_multiple_of(3) {
        return Err("an ASCII STL whose vertices are not in threes".into());
    }
    Ok(verts.chunks_exact(3).map(|t| [t[0], t[1], t[2]]).collect())
}

/// A binary STL of these triangles, for the tests.
#[cfg(test)]
pub fn binary(tris: &[Triangle]) -> Vec<u8> {
    let mut data = vec![0u8; 80];
    data.extend((tris.len() as u32).to_le_bytes());
    for t in tris {
        data.extend([0u8; 12]);
        for x in t.iter().flatten() {
            data.extend((*x as f32).to_le_bytes());
        }
        data.extend([0u8; 2]);
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    const TRIANGLE: Triangle = [[0.0, 0.0, 0.0], [1.5, 0.0, 0.0], [0.0, -2.25, 1e3]];

    #[test]
    fn a_binary_file_is_read() {
        let data = binary(&[TRIANGLE, TRIANGLE]);
        assert!(has_triangles(&data));
        assert_eq!(read(&data).unwrap(), vec![TRIANGLE, TRIANGLE]);
    }

    #[test]
    fn a_binary_file_of_no_triangles_has_none() {
        let data = binary(&[]);
        assert!(!has_triangles(&data));
        assert_eq!(read(&data).unwrap(), Vec::<Triangle>::new());
    }

    #[test]
    fn an_ascii_file_is_read() {
        let text = "solid s\n facet normal 0 0 1\n  outer loop\n   vertex 0 0 0\n   \
                    vertex 1.5 0 0\r\n   vertex\t0 -2.25 1e3\n  endloop\n endfacet\nendsolid s\n";
        assert!(has_triangles(text.as_bytes()));
        assert_eq!(read(text.as_bytes()).unwrap(), vec![TRIANGLE]);
    }

    #[test]
    fn an_ascii_file_of_no_triangles_has_none() {
        assert!(!has_triangles(b"solid s\nendsolid s\n"));
    }

    #[test]
    fn vertices_not_in_threes_are_refused() {
        let text = "solid s vertex 0 0 0 vertex 1 0 0 endsolid s";
        assert!(read(text.as_bytes()).unwrap_err().contains("not in threes"));
    }

    #[test]
    fn a_vertex_that_is_no_number_is_refused() {
        assert!(read(b"vertex 0 zero 0").unwrap_err().contains("no number"));
        assert!(read(b"vertex 0 0").unwrap_err().contains("ends within"));
    }

    /// A file too short of its count is not a binary one, and with no `vertex` holds nothing.
    #[test]
    fn a_truncated_binary_file_is_not_binary() {
        let mut data = binary(&[TRIANGLE]);
        data.pop();
        assert!(!has_triangles(&data));
    }
}
