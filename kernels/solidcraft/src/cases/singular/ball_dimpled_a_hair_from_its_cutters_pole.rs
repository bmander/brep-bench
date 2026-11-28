//! A ball of radius 10 centred on the origin less a ball of radius 0.8 whose centre is 10.4 from
//! the origin, at height 5.501 on the half-plane at 120° from +x toward +y: one dimple. solidcore
//! puts each ball's poles on the vertical through its centre, so the small ball's lower pole lies
//! 0.00008 inside the big ball and the circle where they meet passes about 0.00009 from it.
use crate::bench::cut;
use crate::Refusal;
use solidcore_ops::{make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let (height, angle) = (5.501, 120f64.to_radians());
    let rho = (10.4f64 * 10.4 - height * height).sqrt();
    let ball = make_sphere([0.0, 0.0, 0.0], 10.0)?;
    let cutter = make_sphere([rho * angle.cos(), rho * angle.sin(), height], 0.8)?;
    cut(ball, cutter)
}
