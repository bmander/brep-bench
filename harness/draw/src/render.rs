//! A solid's triangles as drawings: rasterised with a depth buffer, lightly shaded, and outlined
//! where the depth jumps (silhouettes) or the surface turns sharply (creases), as a technical
//! drawing is.
//!
//! The arithmetic is harness/draw.py's, which this replaced, operation for operation and in its
//! order, in the same double precision: a solid's drawings are the same pixels they were (the
//! tests hold it to that). So a sum is not regrouped here, nor a product folded into another.

#[cfg(test)]
use crate::stl::Point;
use crate::stl::Triangle;

/// Pixels per unit of length are chosen so the sheet's views fit in this many pixels across.
const SHEET: usize = 560;
/// The isometric view's size.
const ISO: usize = 320;
/// The thumbnail's: the explorer shows it at 40 pixels, other pages at 64, on screens of up to
/// twice that density.
const THUMB: usize = 160;
/// Rendered at this many times the size and averaged down, so lines are smooth.
const SUPER: usize = 2;
/// A crease is drawn where neighbouring facets turn by more than this many degrees.
const CREASE: f64 = 28.0;
/// Facets turned further from the eye than this (the cosine) draw no crease.
const GRAZING: f64 = 0.2;
/// The pixels left clear around a view of the sheet, and around the thumbnail.
const MARGIN: usize = 12;
const THUMB_MARGIN: usize = 8;

/// How a view is lit and coloured. A facet's tone runs from `dark` to `light` by
/// ambient + diffuse·(n·key), key a light towards the viewer, to its left and up (its coordinates
/// along the view's right, up and towards-the-viewer directions); `shine` (a weight and an
/// exponent) whitens a facet near the key's mirror direction. `ground` is behind the solid, `ink`
/// its lines.
pub struct Look {
    ground: [u8; 3],
    ink: [u8; 3],
    light: [f64; 3],
    dark: [f64; 3],
    key: [f64; 3],
    ambient: f64,
    diffuse: f64,
    shine: Option<(f64, f64)>,
}

/// The sheet's: a technical drawing, lines on white with the faces only told apart.
pub const PAPER: Look = Look {
    ground: [252, 252, 250],
    ink: [34, 36, 40],
    light: [255.0, 255.0, 255.0],
    dark: [196.0, 203.0, 214.0],
    key: [-0.45, 0.6, 1.0],
    ambient: 0.25,
    diffuse: 0.75,
    shine: None,
};

/// The thumbnail's: a near-black ground, no ambient light and a high key, so a face turned from
/// the light goes dark and the solid's form reads at a glance.
pub const GRAPHITE: Look = Look {
    ground: [36, 36, 38],
    ink: [12, 12, 14],
    light: [236.0, 240.0, 246.0],
    dark: [52.0, 60.0, 74.0],
    key: [-0.7, 0.9, 0.6],
    ambient: 0.0,
    diffuse: 1.0,
    shine: Some((0.25, 60.0)),
};

/// A view: the directions to the right, up, and towards the viewer.
pub type Basis = [[f64; 3]; 3];

/// The sheet's three views, by name, in the order they are drawn and listed.
pub const VIEWS: [(&str, Basis); 3] = [
    (
        "front",
        [[1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, -1.0, 0.0]],
    ),
    ("top", [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]),
    ("right", [[0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]]),
];

/// The drawings of a case, by the name each is kept under, in the order they are listed.
pub const DRAWINGS: [&str; 5] = ["front", "top", "right", "iso", "thumb"];

/// An RGB image: its pixels in rows from the top, three bytes each.
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub rgb: Vec<u8>,
}

fn unit(v: [f64; 3]) -> [f64; 3] {
    let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / n, v[1] / n, v[2] / n]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// The larger, and the first where neither is: Python's `max`.
fn max(a: f64, b: f64) -> f64 {
    if b > a { b } else { a }
}

/// The smaller, and the first where neither is: Python's `min`.
fn min(a: f64, b: f64) -> f64 {
    if b < a { b } else { a }
}

/// Looking from the front, right and top at once: towards (−1, 1, −1).
pub fn iso_view() -> Basis {
    let w = unit([1.0, -1.0, 1.0]);
    let r = unit(cross([0.0, 0.0, 1.0], w));
    [r, cross(w, r), w]
}

/// The extent of the triangles (some) along a view's right and up directions: left, right, bottom,
/// top. Each is the first corner's, unless a later one's is less (or more), as Python's `min` and
/// `max` are: so a coordinate that is no number is within the extent or not as it was in draw.py's.
fn bounds(tris: &[Triangle], basis: &Basis) -> [f64; 4] {
    let [r, u, _] = *basis;
    let first = tris[0][0];
    let mut b = [dot(first, r), dot(first, r), dot(first, u), dot(first, u)];
    for p in tris.iter().flatten() {
        let (x, y) = (dot(*p, r), dot(*p, u));
        b = [min(b[0], x), max(b[1], x), min(b[2], y), max(b[3], y)];
    }
    b
}

/// The triangles seen along `basis` at `scale` pixels a unit, `frame` (the view's extent: left,
/// right, bottom, top) framed by `margin` pixels, lit and coloured as `look` says. A view or a
/// facet whose place in it is no number (of a mesh with a coordinate that is none) is refused, as
/// draw.py failed of it.
pub fn render(
    tris: &[Triangle],
    basis: &Basis,
    scale: f64,
    frame: [f64; 4],
    margin: usize,
    look: &Look,
) -> Result<Image, String> {
    let [r, u, w] = *basis;
    let [x0, x1, y0, y1] = frame;
    let k = scale * SUPER as f64;
    let m = (margin * SUPER) as f64;
    let (wide, tall) = (((x1 - x0) * k).ceil(), ((y1 - y0) * k).ceil());
    if !(wide.is_finite() && tall.is_finite()) {
        return Err("a view whose size is not a number".into());
    }
    let width = wide as usize + 2 * margin * SUPER;
    let height = tall as usize + 2 * margin * SUPER;
    if width.saturating_mul(height) > MOST {
        return Err(format!(
            "a view {width} pixels by {height}, too large to draw"
        ));
    }
    // the nearest facet at each pixel, and how near
    let mut depth = vec![f64::NEG_INFINITY; width * height];
    let mut face = vec![NONE; width * height];
    // of each facet drawn: its normal, how far its depth changes from one pixel to the next, and
    // how squarely it faces the eye
    let mut normals: Vec<[f64; 3]> = Vec::new();
    let mut slopes: Vec<f64> = Vec::new();
    let mut facing: Vec<f64> = Vec::new();
    let [kr, ku, kw] = look.key;
    let light = unit([
        kr * r[0] + ku * u[0] + kw * w[0],
        kr * r[1] + ku * u[1] + kw * w[1],
        kr * r[2] + ku * u[2] + kw * w[2],
    ]);
    for &[a, b, c] in tris {
        let n = cross(
            [b[0] - a[0], b[1] - a[1], b[2] - a[2]],
            [c[0] - a[0], c[1] - a[1], c[2] - a[2]],
        );
        let size = dot(n, n).sqrt();
        if size == 0.0 {
            continue; // no area (or too little to square): nothing to see
        }
        let n = [n[0] / size, n[1] / size, n[2] / size];
        // facing away (a closed solid's back) or edge on: judged on the unit normal, whose facing
        // the slope below divides by, so a facet edge on by a rounding error is skipped too
        let nw = dot(n, w);
        if nw <= 0.0 {
            continue;
        }
        let corners = [a, b, c];
        let sx = corners.map(|p| (dot(p, r) - x0) * k + m);
        let sy = corners.map(|p| (y1 - dot(p, u)) * k + m);
        let sz = corners.map(|p| dot(p, w));
        let area = (sx[1] - sx[0]) * (sy[2] - sy[0]) - (sx[2] - sx[0]) * (sy[1] - sy[0]);
        if area == 0.0 {
            continue;
        }
        let fi = normals.len() as u32;
        normals.push(n);
        slopes.push(max(dot(n, r).abs(), dot(n, u).abs()) / (nw * k));
        facing.push(nw);
        // the pixels the facet may cover: those of its box, and one more each way
        let left = min(min(sx[0], sx[1]), sx[2]);
        let right = max(max(sx[0], sx[1]), sx[2]);
        let top = min(min(sy[0], sy[1]), sy[2]);
        let bottom = max(max(sy[0], sy[1]), sy[2]);
        if ![left, right, top, bottom].iter().all(|x| x.is_finite()) {
            return Err("a facet whose place in the view is not a number".into());
        }
        let lo_x = (left as i64).max(0);
        let hi_x = (right as i64 + 1).min(width as i64 - 1);
        let lo_y = (top as i64).max(0);
        let hi_y = (bottom as i64 + 1).min(height as i64 - 1);
        let inv = 1.0 / area;
        for py in lo_y..=hi_y {
            let cy = py as f64 + 0.5;
            let row = py as usize * width;
            for px in lo_x..=hi_x {
                let cx = px as f64 + 0.5;
                // the pixel's centre in the facet's barycentric coordinates
                let l0 = ((sx[1] - cx) * (sy[2] - cy) - (sx[2] - cx) * (sy[1] - cy)) * inv;
                let l1 = ((sx[2] - cx) * (sy[0] - cy) - (sx[0] - cx) * (sy[2] - cy)) * inv;
                let l2 = 1.0 - l0 - l1;
                if l0 < 0.0 || l1 < 0.0 || l2 < 0.0 {
                    continue;
                }
                let z = l0 * sz[0] + l1 * sz[1] + l2 * sz[2];
                let i = row + px as usize;
                if z > depth[i] {
                    depth[i] = z;
                    face[i] = fi;
                }
            }
        }
    }
    // the depth a jump must exceed to be a silhouette: a few pixels' worth of length
    let jump = 3.0 / k;
    let cos_crease = CREASE.to_radians().cos();
    let half = unit([light[0] + w[0], light[1] + w[1], light[2] + w[2]]);
    let tone: Vec<[u8; 3]> = normals
        .iter()
        .map(|&n| {
            let s = min(1.0, look.ambient + look.diffuse * max(0.0, dot(n, light)));
            let mut t = [0, 1, 2].map(|j| look.dark[j] + (look.light[j] - look.dark[j]) * s);
            if let Some((weight, exponent)) = look.shine {
                let h = min(1.0, weight * max(0.0, dot(n, half)).powf(exponent));
                t = t.map(|c| c + (255.0 - c) * h);
            }
            t.map(|c| c as u8)
        })
        .collect();
    let mut big = vec![look.ground; width * height];
    for py in 0..height {
        let row = py * width;
        for px in 0..width {
            let i = row + px;
            let f = face[i];
            if f == NONE {
                continue;
            }
            // the pixels beside, above and below, where the image has them
            let beside = [
                (px > 0).then(|| i - 1),
                (px + 1 < width).then(|| i + 1),
                (py > 0).then(|| i - width),
                (py + 1 < height).then(|| i + width),
            ];
            let (f, at) = (f as usize, depth[i]);
            let edge = beside.into_iter().flatten().any(|j| {
                let g = face[j];
                if g == NONE {
                    return true; // the solid's outline against the ground
                }
                let g = g as usize;
                g != f
                    // a silhouette over a farther surface: a jump more than this facet's own
                    // slope makes from one pixel to the next
                    && (at - depth[j] > jump + slopes[f]
                        // a crease, between facets both turned towards the eye (near a
                        // silhouette a smooth surface's facets crowd into one pixel at steep
                        // turns)
                        || (facing[f] > GRAZING
                            && facing[g] > GRAZING
                            && dot(normals[f], normals[g]) < cos_crease))
            });
            big[i] = if edge { look.ink } else { tone[f] };
        }
    }
    Ok(downsample(&big, width, height))
}

/// No facet: what a pixel of the ground holds for its facet.
const NONE: u32 = u32::MAX;

/// The most pixels a view is drawn of, before it is averaged down: many times the largest a sheet
/// has, and few enough to hold in memory.
const MOST: usize = 1 << 24;

/// The image at one `SUPER`th the size, each pixel the mean of those it is made of.
fn downsample(big: &[[u8; 3]], width: usize, height: usize) -> Image {
    let (w, h) = (width / SUPER, height / SUPER);
    let n = (SUPER * SUPER) as u32;
    let mut rgb = Vec::with_capacity(w * h * 3);
    for y in 0..h {
        for x in 0..w {
            let mut acc = [0u32; 3];
            for dy in 0..SUPER {
                let row = (y * SUPER + dy) * width;
                for dx in 0..SUPER {
                    let p = big[row + x * SUPER + dx];
                    for (sum, c) in acc.iter_mut().zip(p) {
                        *sum += c as u32;
                    }
                }
            }
            rgb.extend(acc.map(|sum| (sum / n) as u8));
        }
    }
    Image {
        width: w,
        height: h,
        rgb,
    }
}

/// The larger of two lengths, and at least a hair, so that a solid of no extent still has a
/// scale.
fn extent(a: f64, b: f64) -> f64 {
    max(max(a, b), 1e-9)
}

/// A case's drawings, each under its name (`DRAWINGS`): the three views at one scale, the
/// isometric view and the thumbnail.
pub fn sheet(tris: &[Triangle]) -> Result<Vec<(&'static str, Image)>, String> {
    if tris.is_empty() {
        return Err("no triangles to draw".into());
    }
    let boxes = VIEWS.map(|(_, basis)| bounds(tris, &basis));
    let [front, top, right] = boxes;
    // front and right side by side, top above front: fit the whole sheet in SHEET pixels
    let across = (front[1] - front[0]) + (right[1] - right[0]);
    let down = (top[3] - top[2]) + (front[3] - front[2]);
    let scale = (SHEET - 4 * MARGIN) as f64 / extent(across, down);
    let mut drawings = Vec::with_capacity(DRAWINGS.len());
    for ((name, basis), frame) in VIEWS.iter().zip(boxes) {
        drawings.push((*name, render(tris, basis, scale, frame, MARGIN, &PAPER)?));
    }
    let iso = iso_view();
    let frame = bounds(tris, &iso);
    let (wide, tall) = (frame[1] - frame[0], frame[3] - frame[2]);
    let iso_scale = (ISO - 2 * MARGIN) as f64 / extent(wide, tall);
    drawings.push(("iso", render(tris, &iso, iso_scale, frame, MARGIN, &PAPER)?));
    // square, the solid centred, so its ground fills the tile it is shown in
    let side = extent(wide, tall);
    let (cx, cy) = ((frame[0] + frame[1]) / 2.0, (frame[2] + frame[3]) / 2.0);
    let square = [
        cx - side / 2.0,
        cx + side / 2.0,
        cy - side / 2.0,
        cy + side / 2.0,
    ];
    let thumb_scale = (THUMB - 2 * THUMB_MARGIN) as f64 / side;
    drawings.push((
        "thumb",
        render(tris, &iso, thumb_scale, square, THUMB_MARGIN, &GRAPHITE)?,
    ));
    Ok(drawings)
}

/// A solid to draw in the tests: a dome on a square block, a smooth surface that silhouettes
/// against itself and creases against its walls, and a small block afloat above it, whose outline
/// falls on the dome. Every coordinate is a small multiple of an eighth, so it is exactly the
/// same solid wherever it is made.
#[cfg(test)]
pub fn dome() -> Vec<Triangle> {
    const N: i32 = 6;
    const FLOOR: f64 = -2.0;
    let top = |x: i32, y: i32| -> Point { [x as f64, y as f64, (72 - x * x - y * y) as f64 / 8.0] };
    let foot = |x: i32, y: i32| -> Point { [x as f64, y as f64, FLOOR] };
    let mut tris = Vec::new();
    for x in -N..N {
        for y in -N..N {
            tris.push([top(x, y), top(x + 1, y), top(x + 1, y + 1)]);
            tris.push([top(x, y), top(x + 1, y + 1), top(x, y + 1)]);
        }
    }
    tris.push([foot(-N, -N), foot(-N, N), foot(N, N)]);
    tris.push([foot(-N, -N), foot(N, N), foot(N, -N)]);
    for t in -N..N {
        // the walls at y = −N, y = N, x = −N and x = N, each facing out
        for (a, b, out) in [
            ((t, -N), (t + 1, -N), true),
            ((t, N), (t + 1, N), false),
            ((-N, t), (-N, t + 1), false),
            ((N, t), (N, t + 1), true),
        ] {
            let (a0, b0, b1, a1) = (foot(a.0, a.1), foot(b.0, b.1), top(b.0, b.1), top(a.0, a.1));
            if out {
                tris.extend([[a0, b0, b1], [a0, b1, a1]]);
            } else {
                tris.extend([[a0, b1, b0], [a0, a1, b1]]);
            }
        }
    }
    tris.extend(block([-2.0, -2.0, 12.0], [2.0, 2.0, 14.0]));
    tris
}

/// A block from one corner to the opposite, its faces facing out.
#[cfg(test)]
pub fn block(lo: Point, hi: Point) -> Vec<Triangle> {
    let p = |i: usize| -> Point {
        [
            if i & 1 == 0 { lo[0] } else { hi[0] },
            if i & 2 == 0 { lo[1] } else { hi[1] },
            if i & 4 == 0 { lo[2] } else { hi[2] },
        ]
    };
    // each face's corners, anticlockwise from outside
    [
        [0, 2, 3, 1],
        [4, 5, 7, 6],
        [0, 1, 5, 4],
        [2, 6, 7, 3],
        [0, 4, 6, 2],
        [1, 3, 7, 5],
    ]
    .into_iter()
    .flat_map(|[a, b, c, d]| [[p(a), p(b), p(c)], [p(a), p(c), p(d)]])
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    fn pixel(image: &Image, x: usize, y: usize) -> [u8; 3] {
        let at = (y * image.width + x) * 3;
        image.rgb[at..at + 3].try_into().unwrap()
    }

    /// The dome's drawings are the pixels harness/draw.py drew of it: each drawing's size, and
    /// the SHA-256 of its pixels, taken from draw.py's own PNGs of the same solid before this
    /// replaced it. A change to the arithmetic that moves a single pixel fails here.
    #[test]
    fn the_drawings_are_the_pixels_draw_py_drew() {
        let expected = [
            (
                "front",
                243,
                317,
                "93334199bb3a41e5cdee4130213c24e2ee7e0636b43b74e74cbedafcc3ef254c",
            ),
            (
                "top",
                243,
                243,
                "bdb92cf1de7df8ea5a1275de518bca7024ea115e1faabab6a32c1f7e02c70d42",
            ),
            (
                "right",
                243,
                317,
                "12653b5ee23d5db3780959205910b76ce52f241c4f018c36c8c8148154dd884c",
            ),
            (
                "iso",
                280,
                320,
                "8f719a041d2bdba6c0c41a36e3997fdcfe080fa0d5ca184f3815e4ac6a724a53",
            ),
            (
                "thumb",
                160,
                160,
                "d9a4a0764294806d9291ea11651013710e094e60a5dd58f493ef7f85d6f1171d",
            ),
        ];
        let drawings = sheet(&dome()).unwrap();
        assert_eq!(drawings.len(), expected.len());
        for ((name, image), (want, width, height, sha)) in drawings.iter().zip(expected) {
            assert_eq!(*name, want);
            assert_eq!(
                (image.width, image.height),
                (width, height),
                "{name}'s size"
            );
            let got: String = Sha256::digest(&image.rgb)
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect();
            assert_eq!(got, sha, "{name}'s pixels");
        }
    }

    #[test]
    fn the_drawings_are_named_in_their_order() {
        let drawings = sheet(&block([0.0; 3], [1.0, 2.0, 3.0])).unwrap();
        let names: Vec<&str> = drawings.iter().map(|(name, _)| *name).collect();
        assert_eq!(names, DRAWINGS);
    }

    /// A block's front view: its ground at the corner, its outline inked, its face inside, and
    /// the block as many pixels across and up as its scale makes it.
    #[test]
    fn a_block_is_drawn_to_scale_and_outlined() {
        let tris = block([0.0, 0.0, 0.0], [4.0, 2.0, 1.0]);
        let (_, basis) = VIEWS[0];
        let image = render(&tris, &basis, 10.0, [0.0, 4.0, 0.0, 1.0], 3, &PAPER).unwrap();
        assert_eq!((image.width, image.height), (46, 16));
        assert_eq!(pixel(&image, 0, 0), PAPER.ground);
        // the face is square to the eye: one tone all over
        let inside = pixel(&image, 23, 8);
        assert_ne!(inside, PAPER.ground);
        assert_eq!(pixel(&image, 10, 6), inside);
        // its outline, a line half as wide as a pixel of the image: the ink and the face, mixed
        let outline = pixel(&image, 3, 8);
        for c in 0..3 {
            assert_eq!(
                outline[c] as u32,
                (PAPER.ink[c] as u32 + inside[c] as u32) / 2
            );
        }
    }

    /// The thumbnail is square whatever the solid's shape, and on its dark ground.
    #[test]
    fn the_thumbnail_is_square() {
        let drawings = sheet(&block([0.0; 3], [9.0, 1.0, 2.0])).unwrap();
        let (_, thumb) = &drawings[4];
        assert_eq!((thumb.width, thumb.height), (THUMB, THUMB));
        assert_eq!(pixel(thumb, 0, 0), GRAPHITE.ground);
    }

    /// A facet edge on to the eye: by rounding it faces the eye before its normal is made a
    /// unit and by nothing after, which the slope of its depth would be divided by.
    #[test]
    fn a_facet_edge_on_to_the_eye_is_passed_over() {
        let iso = iso_view();
        let w = iso[2];
        let mut found = 0;
        // corners a, b and c = a + t·w + a hair: the facet holds the line of sight
        let mut seed = 1u64;
        let mut random = move || {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (seed >> 11) as f64 / (1u64 << 53) as f64
        };
        for _ in 0..200_000 {
            let a = [0, 1, 2].map(|_| random() * 100.0 - 50.0);
            let b = [0, 1, 2].map(|j| a[j] + random() * 10.0 - 5.0);
            let t = 0.1 + random() * 4.9;
            let c = [0, 1, 2].map(|j| a[j] + t * w[j] + (random() - 0.5) * 2e-15);
            let n = cross(
                [b[0] - a[0], b[1] - a[1], b[2] - a[2]],
                [c[0] - a[0], c[1] - a[1], c[2] - a[2]],
            );
            if dot(n, w) > 0.0 && dot(unit(n), w) == 0.0 {
                found += 1;
                let image = render(
                    &[[a, b, c]],
                    &iso,
                    2.0,
                    [-60.0, 60.0, -60.0, 60.0],
                    4,
                    &PAPER,
                )
                .unwrap();
                assert!(image.rgb.chunks_exact(3).all(|p| p == PAPER.ground));
                if found == 20 {
                    break;
                }
            }
        }
        assert_eq!(found, 20, "too few facets of the kind were made to try");
    }

    #[test]
    fn nothing_to_draw_is_refused() {
        assert!(sheet(&[]).err().unwrap().contains("no triangles"));
    }

    /// A coordinate that is no number is as it was to draw.py, whose least and most of some
    /// numbers were the first unless a later one was less or more: the first corner's makes the
    /// view's size no number, which is refused; a later one's is outside every comparison, and
    /// its facet covers no pixel, unless its first corner's place is no number, which is refused.
    #[test]
    fn a_coordinate_that_is_no_number_is_as_it_was_to_draw_py() {
        let tris = block([0.0; 3], [1.0; 3]);
        let size = |tris: &[Triangle]| sheet(tris).map(|d| (d[0].1.width, d[0].1.height));
        let whole = size(&tris).unwrap();

        let mut first = tris.clone();
        first[0][0][0] = f64::NAN;
        assert!(size(&first).err().unwrap().contains("size is not a number"));

        let mut later = tris.clone();
        later[3][2][0] = f64::NAN;
        assert_eq!(size(&later).unwrap(), whole);

        let mut leading = tris.clone();
        leading[3][0][0] = f64::NAN;
        assert!(
            size(&leading)
                .err()
                .unwrap()
                .contains("place in the view is not a number")
        );
    }

    /// A solid of no extent (every corner at one point) is still drawn, as its ground.
    #[test]
    fn a_solid_of_no_extent_is_drawn_as_its_ground() {
        let drawings = sheet(&[[[1.0, 2.0, 3.0]; 3]]).unwrap();
        for (name, image) in &drawings {
            let ground = if *name == "thumb" {
                GRAPHITE.ground
            } else {
                PAPER.ground
            };
            assert!(image.rgb.chunks_exact(3).all(|p| p == ground), "{name}");
        }
    }
}
