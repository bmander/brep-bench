//! The solids and picks the cases share: a closed polygon as a profile, a block between two
//! corners, the L-block, a ball cut by a level, a rounded plate, a capsule, a drilled block, a bent
//! flange and a half pipe, and a face picked by a point inside it. Everything else a case builds
//! with cad-kernel's own `build` functions.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, kept, prism, revolve, sphere, Profile, ProfileEdge};
use cad_kernel::geom::{tolerance, Frame, V};
use cad_kernel::query::{Located, Place};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{add, cross, distance, unit};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// The closed polygon through `points` as a profile of lines, its normal by Newell's rule (so a
/// polygon walked counter-clockwise seen from above faces up).
pub fn polygon(points: &[V]) -> Profile {
    let n = points.len();
    let normal = (0..n).fold([0.; 3], |s, k| add(s, cross(points[k], points[(k + 1) % n])));
    let edges = (0..n).map(|k| ProfileEdge::Line { a: points[k], b: points[(k + 1) % n] }).collect();
    Profile { origin: points[0], normal: unit(normal), loops: vec![edges], names: Vec::new() }
}

/// The block with its least corner at `lo` and its greatest at `hi`: its floor, a rectangle,
/// swept up.
pub fn block(lo: V, hi: V) -> Result<Brep, String> {
    let floor = polygon(&[lo, [hi[0], lo[1], lo[2]], [hi[0], hi[1], lo[2]], [lo[0], hi[1], lo[2]]]);
    prism(&floor, 0., hi[2] - lo[2])
}

/// The L-shaped plate through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), 8 high.
pub fn l_block() -> Result<Brep, String> {
    let l = polygon(&[[0., 0., 0.], [20., 0., 0.], [20., 5., 0.], [5., 5., 0.], [5., 15., 0.], [0., 15., 0.]]);
    prism(&l, 0., 8.)
}

/// The faces of `b` through which the points `on` pass, one each: a face picked by a point inside
/// it, as a description names a face by where it is.
pub fn faces_through(b: &Brep, on: &[V]) -> Result<Vec<usize>, String> {
    if on.is_empty() { return Ok(Vec::new()); }
    let tol = tolerance(b.size());
    let located = Located::new(b, tol);
    on.iter().map(|&p| {
        (0..b.faces.len()).find(|&fi| {
            // (a face whose box the point is clear of is not read onto)
            located.face_gap(fi, p) <= tol
                && distance(b.faces[fi].surface.point(located.uv(fi, p)), p) <= 1e-9 * (1. + b.size())
                && located.face_place(fi, p) == Place::In
        }).ok_or_else(|| format!("no face through {p:?}"))
    }).collect()
}

/// The ball of radius `r` about `c` kept below the plane `z = h` (its axis along z, so the plane
/// cuts it along one of its parallels).
pub fn ball_below(c: V, r: f64, h: f64) -> Result<Brep, String> {
    kept(&sphere(c, [0., 0., 1.], r)?, [c[0], c[1], h], [0., 0., -1.], 0.)
}

/// The plate whose outline is the rectangle from `lo` to `hi` in the plane z = 0, its corners
/// rounded to `r`, `depth` high.
pub fn rounded_plate(lo: [f64; 2], hi: [f64; 2], r: f64, depth: f64) -> Result<Brep, String> {
    let corners = [([hi[0] - r, lo[1] + r], -FRAC_PI_2), ([hi[0] - r, hi[1] - r], 0.),
                   ([lo[0] + r, hi[1] - r], FRAC_PI_2), ([lo[0] + r, lo[1] + r], PI)];
    let mut edges = Vec::new();
    for k in 0..4 {
        let (c, a) = corners[k];
        let frame = Frame::new([c[0], c[1], 0.], [0., 0., 1.], [1., 0., 0.]);
        edges.push(ProfileEdge::Arc { frame, r, span: Some([a, a + FRAC_PI_2]) });
        let end = [c[0] + r * (a + FRAC_PI_2).cos(), c[1] + r * (a + FRAC_PI_2).sin(), 0.];
        let (n, b) = corners[(k + 1) % 4];
        let start = [n[0] + r * b.cos(), n[1] + r * b.sin(), 0.];
        edges.push(ProfileEdge::Line { a: end, b: start });
    }
    prism(&Profile { origin: [0., 0., 0.], normal: [0., 0., 1.], loops: vec![edges], names: Vec::new() }, 0., depth)
}

/// The capsule's end: the cylinder of radius `r` on the z axis from z = 0 to `h`, capped by the
/// half ball of radius `r` about (0, 0, h), its section turned about z.
pub fn capsule(r: f64, h: f64) -> Result<Brep, String> {
    let arc = ProfileEdge::Arc { frame: Frame::new([0., 0., h], [0., -1., 0.], [1., 0., 0.]), r, span: Some([0., FRAC_PI_2]) };
    let edges = vec![ProfileEdge::Line { a: [0., 0., 0.], b: [r, 0., 0.] },
                     ProfileEdge::Line { a: [r, 0., 0.], b: [r, 0., h] }, arc,
                     ProfileEdge::Line { a: [0., 0., h + r], b: [0., 0., 0.] }];
    revolve(&Profile { origin: [0., 0., 0.], normal: [0., -1., 0.], loops: vec![edges], names: Vec::new() },
            [0., 0., 0.], [0., 0., 1.], TAU)
}

/// The bent flange's solid: the profile from (−10, 0, 0) to the origin, round the quarter circle
/// of radius 2 about (0, 0, 2) to (2, 0, 2) and up to (2, 0, 12), closed above and to the left,
/// swept 8 along y; and the points its floor, bend and wall are picked by.
pub fn bent_flange() -> Result<(Brep, [V; 3]), String> {
    let bend = Frame::new([0., 0., 2.], [0., 1., 0.], [1., 0., 0.]);
    let section = Profile { origin: [0., 0., 0.], normal: [0., 1., 0.], names: Vec::new(), loops: vec![vec![
        ProfileEdge::Line { a: [-10., 0., 0.], b: [0., 0., 0.] },
        ProfileEdge::Arc { frame: bend, r: 2., span: Some([0., FRAC_PI_2]) },
        ProfileEdge::Line { a: [2., 0., 2.], b: [2., 0., 12.] },
        ProfileEdge::Line { a: [2., 0., 12.], b: [-10., 0., 12.] },
        ProfileEdge::Line { a: [-10., 0., 12.], b: [-10., 0., 0.] },
    ]] };
    let h = std::f64::consts::SQRT_2;
    Ok((prism(&section, 0., 8.)?, [[-5., 4., 0.], [h, 4., 2. - h], [2., 4., 7.]]))
}

/// The half of the cylinder of radius 5 on the z axis from z = 0 to 10 where y ≥ 0.
pub fn half_pipe() -> Result<Brep, String> {
    kept(&prism(&disc([0., 0., 0.], [0., 0., 1.], 5.), 0., 10.)?, [0., 0., 0.], [0., 1., 0.], 0.)
}

/// The block from the origin to (20, 20, 10) with a hole of radius 3 bored right through it along
/// z, its axis the line x = `x`, y = 10.
pub fn drilled(x: f64) -> Result<Brep, String> {
    let drill = prism(&disc([x, 10., -1.], [0., 0., 1.], 3.), 0., 12.)?;
    Ok(combined(&block([0., 0., 0.], [20., 20., 10.])?, &drill, Op::Cut, 0.)?)
}
