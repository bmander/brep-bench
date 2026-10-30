//! A ball's cap turned once about z from the region in the xz plane between the z axis from −5 to
//! 10, the floor z = −5 out to 5√3 and the rational quadratic Bézier arc (5√3, 0, −5),
//! (10√3, 0, 10), (0, 0, 10) weighted 1, ½, 1.
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::V;
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;
use std::sync::Arc;

const ROOT3: f64 = 1.7320508075688772;
const FLOOR: f64 = -5.;
const TOP: f64 = 10.;
const CONTROL: [V; 3] = [[5. * ROOT3, 0., FLOOR], [10. * ROOT3, 0., TOP], [0., 0., TOP]];
const WEIGHTS: [f64; 3] = [1., 0.5, 1.];

pub fn build() -> Result<Brep, String> {
    // the weighted arc as cad-kernel holds it: a rational quadratic B-spline on clamped knots
    let knots = vec![0., 0., 0., 1., 1., 1.];
    let arc = BSpline::rational(2, knots, CONTROL.to_vec(), WEIGHTS.to_vec())?;
    let (foot, rim) = ([0., 0., FLOOR], CONTROL[0]);
    // the region in the xz plane: out along the floor to the rim, over the arc to the top of the
    // axis, and down the axis
    let section = Profile {
        origin: foot,
        normal: [0., -1., 0.],
        loops: vec![vec![
            ProfileEdge::Line { a: foot, b: rim },
            ProfileEdge::Spline(Arc::new(arc)),
            ProfileEdge::Line { a: CONTROL[2], b: foot },
        ]],
        names: Vec::new(),
    };
    revolve(&section, foot, [0., 0., 1.], TAU)
}
