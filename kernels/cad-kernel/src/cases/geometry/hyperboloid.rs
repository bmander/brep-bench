//! The solid inside the hyperboloid x² + y² = 16(1 + z²/36) between z = −6 and z = 6: the
//! hyperbola x²/16 − z²/36 = 1 of the xz plane turned once about z, its ends flat.
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;
use std::sync::Arc;

const A: f64 = 4.;
const C: f64 = 6.;
const H: f64 = 6.;

pub fn build() -> Result<Brep, String> {
    // a profile has no hyperbola: its arc (A cosh t, C sinh t) for |t| ≤ T, sinh T = H/C (= 1),
    // as one rational quadratic Bézier, its middle pole where the end tangents meet, (A/cosh T, 0),
    // weighted cosh T (= √2)
    let cosh = (1. + H * H / (C * C)).sqrt();
    let (low, high) = ([A * cosh, 0., -H], [A * cosh, 0., H]);
    let middle = [A / cosh, 0., 0.];
    let arc = BSpline::rational(2, vec![0., 0., 0., 1., 1., 1.], vec![low, middle, high],
                                vec![1., cosh, 1.])?;
    let (top, bottom) = ([0., 0., H], [0., 0., -H]);
    // up the side, in along the top, down the axis, out along the floor: counter-clockwise
    // about −y
    let section = Profile {
        origin: bottom,
        normal: [0., -1., 0.],
        loops: vec![vec![
            ProfileEdge::Spline(Arc::new(arc)),
            ProfileEdge::Line { a: high, b: top },
            ProfileEdge::Line { a: top, b: bottom },
            ProfileEdge::Line { a: bottom, b: low },
        ]],
        names: Vec::new(),
    };
    revolve(&section, [0., 0., 0.], [0., 0., 1.], TAU)
}
