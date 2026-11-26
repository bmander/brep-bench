//! The elliptic cylinder x²/100 + y²/4 ≤ 1 from z = 0 to z = 10, grown by 1: the ellipse curves
//! more tightly at the ends of its long axis (radius 0.4) than the distance grown.
use cad_kernel::build::{prism, Profile, ProfileEdge};
use cad_kernel::nurbs::BSpline;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_1_SQRT_2;
use std::sync::Arc;

const A: f64 = 10.;
const B: f64 = 2.;
const HEIGHT: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    // a profile has no ellipse: the rim as a closed rational quadratic B-spline, four quarters,
    // each on the corner of the box round it, weighted √2/2
    let at = |x: f64, y: f64| [A * x, B * y, 0.];
    let poles = vec![at(1., 0.), at(1., 1.), at(0., 1.), at(-1., 1.), at(-1., 0.), at(-1., -1.),
                     at(0., -1.), at(1., -1.), at(1., 0.)];
    let h = FRAC_1_SQRT_2;
    let weights = vec![1., h, 1., h, 1., h, 1., h, 1.];
    let knots = vec![0., 0., 0., 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1., 1., 1.];
    let rim = BSpline::rational(2, knots, poles, weights)?;
    let end = Profile { origin: [0., 0., 0.], normal: [0., 0., 1.],
                        loops: vec![vec![ProfileEdge::Spline(Arc::new(rim))]], names: Vec::new() };
    grown(&prism(&end, 0., HEIGHT)?, 1., Join::Round)
}
