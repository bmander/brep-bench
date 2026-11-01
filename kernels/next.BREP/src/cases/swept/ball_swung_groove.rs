//! The box from (−6, −6, −8) to (18, 18, 0), less everything a ball of radius 3 passes through as
//! its centre swings a quarter turn about the z axis, right-handed, from (12, 0, 0) to (0, 12, 0).
//!
//! next.BREP has no sweep of a solid. A ball's sweep is, by elementary geometry, the tube of the
//! ball's radius about its centre's path with a ball at each end: about a quarter circle, the tube
//! is the ball's great circle in the xz plane revolved a quarter turn about the z axis, and the
//! balls are the kernel's spheres; their union is cut from the block.
use crate::solids::p;
use crate::solids::sweep::circle;
use crate::Refusal;
use brep_kernel::{
    boolean_operation, boolean_operation_nary, make_box_brep, make_sphere_brep,
    revolve_profile_brep, BooleanOperation, BooleanOptions, BrepSolid,
};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(-6.0, -6.0, -8.0), 24.0, 24.0, 8.0)?;
    let up = p(0.0, 0.0, 1.0);
    let section = circle(p(12.0, 0.0, 0.0), p(1.0, 0.0, 0.0), up, 3.0)?;
    let tube = revolve_profile_brep(&section, p(0.0, 0.0, 0.0), up, FRAC_PI_2)?;
    let start = make_sphere_brep(p(12.0, 0.0, 0.0), 3.0, up)?;
    let end = make_sphere_brep(p(0.0, 12.0, 0.0), 3.0, up)?;
    let swept = boolean_operation_nary(&[tube, start, end], BooleanOperation::Union)?;
    Ok(boolean_operation(&block, &swept, BooleanOperation::Subtract, &BooleanOptions::default())?)
}
