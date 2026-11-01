//! A ring about z: the periodic uniform quadratic B-spline in the xz plane whose control polygon is
//! the square of corners (13, 0, 3), (7, 0, 3), (7, 0, −3), (13, 0, −3), turned once about z.
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::V;
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;
use std::sync::Arc;

/// (x, z), taken in order and closing on the first.
const CORNERS: [[f64; 2]; 4] = [[13., 3.], [7., 3.], [7., -3.], [13., -3.]];

pub fn build() -> Result<Brep, String> {
    // cad-kernel's B-splines are clamped: the periodic curve written as its four parabolic arcs,
    // each from one side's middle over a corner to the next side's middle, one clamped quadratic
    // with a double knot at each side's middle
    let at = |p: [f64; 2]| -> V { [p[0], 0., p[1]] };
    let mid = |k: usize| {
        let (a, b) = (CORNERS[k % 4], CORNERS[(k + 1) % 4]);
        at([(a[0] + b[0]) / 2., (a[1] + b[1]) / 2.])
    };
    let mut poles = vec![mid(3)];
    for k in 0..4 {
        poles.push(at(CORNERS[k]));
        poles.push(mid(k));
    }
    let knots = vec![0., 0., 0., 1., 1., 2., 2., 3., 3., 4., 4., 4.];
    let curve = BSpline::new(2, knots, poles)?;
    // the corners run counter-clockwise about −y, as x × z is
    let section = Profile { origin: at([10., 0.]), normal: [0., -1., 0.],
                            loops: vec![vec![ProfileEdge::Spline(Arc::new(curve))]],
                            names: Vec::new() };
    revolve(&section, [0., 0., 0.], [0., 0., 1.], TAU)
}
