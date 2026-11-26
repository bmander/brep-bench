//! The ball of radius 10 about the origin less the ball of radius 0.8 about the point 10.4 from
//! the origin at height z = 5.501 on the half-plane at 120° from +x toward +y: one dimple. Each
//! ball's poles are on the vertical through its center, so the circle where the spheres meet
//! passes about 0.00009 from the small ball's lower pole.
//!
//! Each ball is cadrs's sphere primitive, its poles on the vertical through its center; the small
//! one is cut from the big one.
use crate::bench::{ball, cut, p3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let (h, d): (f64, f64) = (5.501, 10.4);
    let rho = (d * d - h * h).sqrt();
    let a = 120f64.to_radians();
    let big = ball(k, p3(0.0, 0.0, 0.0), 10.0)?;
    let small = ball(k, p3(rho * a.cos(), rho * a.sin(), h), 0.8)?;
    cut(k, big, small)
}
