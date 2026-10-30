//! A dome turned once about z from the region in the xz plane between the z axis, the x axis out to
//! 9 and the cubic Bézier curve (9, 0, 0), (12, 0, 4), (9, 0, 10), (0, 0, 10).
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::V;
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;
use std::sync::Arc;

const CONTROL: [V; 4] = [[9., 0., 0.], [12., 0., 4.], [9., 0., 10.], [0., 0., 10.]];
const ORIGIN: V = [0., 0., 0.];

pub fn build() -> Result<Brep, String> {
    // the curve as cad-kernel holds a Bézier segment: a cubic B-spline on clamped knots
    let knots = vec![0., 0., 0., 0., 1., 1., 1., 1.];
    let curve = BSpline::new(3, knots, CONTROL.to_vec())?;
    let (rim, top) = (CONTROL[0], CONTROL[3]);
    // the region in the xz plane: out along the base to the rim, up the curve to the top of the
    // axis, and down the axis
    let section = Profile {
        origin: ORIGIN,
        normal: [0., -1., 0.],
        loops: vec![vec![
            ProfileEdge::Line { a: ORIGIN, b: rim },
            ProfileEdge::Spline(Arc::new(curve)),
            ProfileEdge::Line { a: top, b: ORIGIN },
        ]],
        names: Vec::new(),
    };
    revolve(&section, ORIGIN, [0., 0., 1.], TAU)
}
