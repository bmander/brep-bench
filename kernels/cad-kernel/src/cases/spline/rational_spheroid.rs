//! A spheroid turned once about z from the region in the xz plane between the z axis from −10 to
//! 10 and the rational quadratic B-spline (0, 0, −10), (6, 0, −10), (6, 0, 0), (6, 0, 10),
//! (0, 0, 10) weighted 1, √2/2, 1, √2/2, 1 on the knots (0, 0, 0, ½, ½, 1, 1, 1).
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::V;
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_1_SQRT_2, TAU};
use std::sync::Arc;

const CONTROL: [V; 5] =
    [[0., 0., -10.], [6., 0., -10.], [6., 0., 0.], [6., 0., 10.], [0., 0., 10.]];
const WEIGHTS: [f64; 5] = [1., FRAC_1_SQRT_2, 1., FRAC_1_SQRT_2, 1.];
const KNOTS: [f64; 8] = [0., 0., 0., 0.5, 0.5, 1., 1., 1.];

pub fn build() -> Result<Brep, String> {
    // the half ellipse as one rational quadratic B-spline, its double knot joining two quarters
    let curve = BSpline::rational(2, KNOTS.to_vec(), CONTROL.to_vec(), WEIGHTS.to_vec())?;
    let (bottom, top) = (CONTROL[0], CONTROL[4]);
    // the region in the xz plane: up the curve from the bottom pole to the top, down the axis
    let section = Profile {
        origin: bottom,
        normal: [0., -1., 0.],
        loops: vec![vec![
            ProfileEdge::Spline(Arc::new(curve)),
            ProfileEdge::Line { a: top, b: bottom },
        ]],
        names: Vec::new(),
    };
    revolve(&section, bottom, [0., 0., 1.], TAU)
}
