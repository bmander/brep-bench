//! A prism from z = 0 to 10 on the periodic uniform quadratic B-spline whose control polygon is the
//! square of corners (±10, ±10).
use cad_kernel::build::{prism, Profile, ProfileEdge};
use cad_kernel::geom::V;
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::sync::Arc;

const CORNERS: [[f64; 2]; 4] = [[10., 10.], [-10., 10.], [-10., -10.], [10., -10.]];
const HEIGHT: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    // cad-kernel's B-splines are clamped: the periodic curve written as its four parabolic arcs,
    // each from one side's middle over a corner to the next side's middle, one clamped quadratic
    // with a double knot at each side's middle
    let at = |p: [f64; 2]| -> V { [p[0], p[1], 0.] };
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
    let end = Profile { origin: [0., 0., 0.], normal: [0., 0., 1.],
                        loops: vec![vec![ProfileEdge::Spline(Arc::new(curve))]],
                        names: Vec::new() };
    prism(&end, 0., HEIGHT)
}
