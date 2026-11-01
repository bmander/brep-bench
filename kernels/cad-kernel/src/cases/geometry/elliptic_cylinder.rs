//! The ellipse x²/25 + y²/9 ≤ 1 in the plane z = 0 extruded up to z = 10.
use cad_kernel::build::{prism, Profile, ProfileEdge};
use cad_kernel::geom::V;
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_1_SQRT_2;
use std::sync::Arc;

const A: f64 = 5.;
const B: f64 = 3.;
const HEIGHT: f64 = 10.;

/// The ellipse of semi-axes `a` along x and `b` along y about `c`, as a closed rational quadratic
/// B-spline: four quarters, each on the corner of the box round it, weighted √2/2.
fn ellipse(c: V, a: f64, b: f64) -> Result<BSpline, String> {
    let at = |x: f64, y: f64| [c[0] + a * x, c[1] + b * y, c[2]];
    let poles = vec![at(1., 0.), at(1., 1.), at(0., 1.), at(-1., 1.), at(-1., 0.), at(-1., -1.),
                     at(0., -1.), at(1., -1.), at(1., 0.)];
    let h = FRAC_1_SQRT_2;
    let weights = vec![1., h, 1., h, 1., h, 1., h, 1.];
    let knots = vec![0., 0., 0., 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1., 1., 1.];
    BSpline::rational(2, knots, poles, weights)
}

pub fn build() -> Result<Brep, String> {
    // a profile has no ellipse: the rim as a rational B-spline, exactly
    let rim = ellipse([0., 0., 0.], A, B)?;
    let end = Profile { origin: [0., 0., 0.], normal: [0., 0., 1.],
                        loops: vec![vec![ProfileEdge::Spline(Arc::new(rim))]], names: Vec::new() };
    prism(&end, 0., HEIGHT)
}
