//! The rod of radius 5 about the z axis from z = 0 to z = 20, less everything a ball of radius 1
//! passes through as its centre follows the helix of radius 5 about the z axis from (5, 0, −2),
//! turning right-handed and rising 4 a turn for six turns, to (5, 0, 22).
//!
//! next.BREP has no sweep of a solid. A ball's sweep is, by elementary geometry, the tube of the
//! ball's radius about its centre's path with a ball at each end: the tube is the kernel's helical
//! sweep of the ball's great circle (which starts its helix on +y, so it is turned a quarter turn
//! back about z and lowered to start at (5, 0, −2)), and the balls are its spheres; their union
//! is cut from the rod.
use crate::solids::p;
use crate::solids::sweep::{circle, moved, turn_z};
use crate::Refusal;
use brep_kernel::{
    boolean_operation, boolean_operation_nary, make_cylinder_brep, make_sphere_brep,
    sweep_profile_helix, BooleanOperation, BooleanOptions, BrepSolid,
};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    let up = p(0.0, 0.0, 1.0);
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), up, 5.0, 20.0)?;
    let section = circle(p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), 1.0)?;
    let coil = sweep_profile_helix(&section, p(0.0, 0.0, 0.0), up, 5.0, 4.0, 6.0, None)?;
    let tube = moved(&coil, turn_z(-FRAC_PI_2, p(0.0, 0.0, -2.0))?)?;
    let start = make_sphere_brep(p(5.0, 0.0, -2.0), 1.0, up)?;
    let end = make_sphere_brep(p(5.0, 0.0, 22.0), 1.0, up)?;
    let swept = boolean_operation_nary(&[tube, start, end], BooleanOperation::Union)?;
    Ok(boolean_operation(&rod, &swept, BooleanOperation::Subtract, &BooleanOptions::default())?)
}
