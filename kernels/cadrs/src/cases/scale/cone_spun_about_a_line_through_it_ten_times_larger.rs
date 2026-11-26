//! The solid a cone fills as it turns 150° about the z axis, which crosses its axis aslant at the
//! origin, inside it (`swept/cone-spun-about-a-line-through-it` ten times larger). The cone's
//! axis runs along w = (cos 30°, 0, sin 30°); its point is at −20w and its base, of radius
//! 70 tan 25°, at +50w.
//!
//! The cone is cadrs's cone primitive; its turn is cadrs's sweep of a solid (one turn).
use crate::bench::{cone, P3, V3};
use crate::shared::swept::{swung, z_axis};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let w = V3::new(30f64.to_radians().cos(), 0.0, 30f64.to_radians().sin());
    let point = P3::from(w * -20.0);
    let c = cone(k, point, w, 0.0, 70.0 * 25f64.to_radians().tan(), 70.0)?;
    swung(k, c, z_axis(), 0.0, 150.0)
}
