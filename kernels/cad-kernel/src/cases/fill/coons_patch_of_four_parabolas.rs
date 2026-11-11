//! The solid over the square from (0, 0) to (10, 10), from z = 0 up to the bilinearly blended
//! Coons patch of four quadratic Bézier arcs at z = 10 at the square's corners: on y = 0 and x = 0
//! arching to 12 through the control points (5, 0, 14) and (0, 5, 14), on x = 10 and y = 10
//! sagging to 8 through (10, 5, 6) and (5, 10, 6).
//!
//! Refused, as `fill/coons-patch-of-four-lines` is: cad-kernel 0.8.0 has no fill of a loop of
//! curves. The four arcs are built as the kernel's B-splines and the walls and floor under them as
//! its planar faces; the missing operation is
//! `nurbs::coons(edges: [&BSpline; 4]) -> Result<Net, String>`, the patch's net with the arcs as
//! its boundary rows, which `sheet` would make a face of and `sew::sewn` close with the rest.
use super::coons_patch_of_four_lines::{bezier, no_fill, walls};
use cad_kernel::topo::Brep;

/// Each arc's control points, end to end round the square.
const ARCS: [[[f64; 3]; 3]; 4] = [
    [[0., 0., 10.], [5., 0., 14.], [10., 0., 10.]],
    [[10., 0., 10.], [10., 5., 6.], [10., 10., 10.]],
    [[10., 10., 10.], [5., 10., 6.], [0., 10., 10.]],
    [[0., 10., 10.], [0., 5., 14.], [0., 0., 10.]],
];

pub fn build() -> Result<Brep, String> {
    let edges = [bezier(&ARCS[0])?, bezier(&ARCS[1])?, bezier(&ARCS[2])?, bezier(&ARCS[3])?];
    Err(no_fill(&walls(&edges)?))
}
