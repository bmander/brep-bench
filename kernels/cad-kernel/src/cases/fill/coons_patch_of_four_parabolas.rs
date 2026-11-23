//! The solid over the square from (0, 0) to (10, 10), from z = 0 up to the bilinearly blended
//! Coons patch of four quadratic Bézier arcs at z = 10 at the square's corners: on y = 0 and x = 0
//! arching to 12 through the control points (5, 0, 14) and (0, 5, 14), on x = 10 and y = 10
//! sagging to 8 through (10, 5, 6) and (5, 10, 6).
//!
//! The kernel fills the loop (`nurbs::coons`): opposite arcs made one space, the patch is exactly
//! a biquadratic net whose boundary rows are the arcs' own poles, which `sheet` makes a face of
//! and `sew::sewn` closes with the walls and the floor under it.
use super::coons_patch_of_four_lines::{bezier, capped};
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
    capped(&edges)
}
