//! The straight bevel case's blank with one tooth space cut by its tooth turned about the z axis
//! by φ (sin φ = 24/(75 − 5√5)), carried by a pinion whose axis is the line through (0, 12, 0)
//! along x: the pinion turns by t about it while the gear turns by −7t/15 about the z axis, t
//! from −70° to 70°.
//!
//! The tooth as in the straight bevel case, turned; cadrs's sweep of a solid under the two turns
//! (`SolidMotion::Rigid`, seen from the gear: by t about the pinion's axis, then by 7t/15 about
//! the z axis), cut from the blank.
use crate::bench::{p3, turned, v3};
use crate::shared::swept::{bevel_cut, bevel_tooth, z_axis};
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let phi = (24.0 / (75.0 - 5.0 * 5f64.sqrt())).asin();
    let tooth = bevel_tooth(k)?;
    let tooth = turned(k, tooth, z_axis(), phi.to_degrees())?;
    bevel_cut(k, tooth, Axis::new(p3(0.0, 12.0, 0.0), v3(1.0, 0.0, 0.0)), -7.0 / 15.0, (-70.0, 70.0))
}
