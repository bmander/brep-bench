//! The two sketches the cases share: a closed polygon as a profile, and a block between two
//! corners. Everything else a case builds with cad-kernel's own `build` functions.
use cad_kernel::build::{prism, Profile, ProfileEdge};
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{add, cross, unit};

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
