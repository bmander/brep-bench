//! The block −12 ≤ x ≤ 12, −8 ≤ y ≤ 8, 0 ≤ z ≤ 8 with the elliptic hole x²/36 + y²/9 < 1 bored
//! through it along z.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{prism, Profile, ProfileEdge};
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_1_SQRT_2;
use std::sync::Arc;

const DEPTH: f64 = 8.;

pub fn build() -> Result<Brep, String> {
    let plate = block([-12., -8., 0.], [12., 8., DEPTH])?;
    // a profile has no ellipse: the hole's rim as a closed rational quadratic B-spline, four
    // quarters each on the corner of the box round it, weighted √2/2, at z = −1
    let at = |x: f64, y: f64| [6. * x, 3. * y, -1.];
    let poles = vec![at(1., 0.), at(1., 1.), at(0., 1.), at(-1., 1.), at(-1., 0.), at(-1., -1.),
                     at(0., -1.), at(1., -1.), at(1., 0.)];
    let h = FRAC_1_SQRT_2;
    let knots = vec![0., 0., 0., 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1., 1., 1.];
    let rim = BSpline::rational(2, knots, poles, vec![1., h, 1., h, 1., h, 1., h, 1.])?;
    let section = Profile { origin: [0., 0., -1.], normal: [0., 0., 1.],
                            loops: vec![vec![ProfileEdge::Spline(Arc::new(rim))]],
                            names: Vec::new() };
    // the drill runs past both faces, from z = −1 to z = 9
    let drill = prism(&section, 0., DEPTH + 2.)?;
    Ok(combined(&plate, &drill, Op::Cut, 0.)?)
}
