//! An oblate spheroid about z: the ellipse x²/36 + z²/9 ≤ 1 of the xz plane turned once about z.
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::V;
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_1_SQRT_2, TAU};
use std::sync::Arc;

const EQUATOR: f64 = 6.;
const POLE: f64 = 3.;

pub fn build() -> Result<Brep, String> {
    // a profile has no ellipse: the half ellipse from pole to pole as two rational quadratic
    // quarters, each on the corner of the box round its quarter, weighted √2/2
    let (south, north): (V, V) = ([0., 0., -POLE], [0., 0., POLE]);
    let poles = vec![south, [EQUATOR, 0., -POLE], [EQUATOR, 0., 0.], [EQUATOR, 0., POLE], north];
    let weights = vec![1., FRAC_1_SQRT_2, 1., FRAC_1_SQRT_2, 1.];
    let arc = BSpline::rational(2, vec![0., 0., 0., 0.5, 0.5, 1., 1., 1.], poles, weights)?;
    // up the arc on the side x ≥ 0, back down the axis: counter-clockwise about −y
    let section = Profile {
        origin: south,
        normal: [0., -1., 0.],
        loops: vec![vec![
            ProfileEdge::Spline(Arc::new(arc)),
            ProfileEdge::Line { a: north, b: south },
        ]],
        names: Vec::new(),
    };
    revolve(&section, [0., 0., 0.], [0., 0., 1.], TAU)
}
