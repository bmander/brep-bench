//! A hollow square duct narrowing round a quarter bend of radius 30 about the z axis, from
//! (30, 0, 0) to (0, 30, 0): its section, in the vertical plane through the z axis, the square of
//! half-width h = 9 − 3·θ/90° centred on the arc, with a hole of half-width h − 2.
//!
//! The kernel's sweep with a scaling law (`sweep_profile_twisted_scaled`, no twist): the section
//! where the bend starts, drawn in the xz plane about (30, 0, 0), swept along the arc and scaled
//! about its centre evenly with the distance gone, to 6/9 of its size at the end for the outside
//! and to 4/7 for the hole, which is cut away.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{
    boolean_operation, make_arc, sweep_profile_twisted_scaled, BooleanOperation, BooleanOptions,
    BrepSolid,
};
use std::f64::consts::FRAC_PI_2;

/// The square of half-width `h` about (30, 0, 0) in the xz plane, swept round the bend and scaled
/// to the half-width `end` where it stops.
fn duct(h: f64, end: f64) -> Result<BrepSolid, Refusal> {
    let (o, x, y) = (p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0));
    let square = polygon(&[p(30.0 - h, 0.0, -h), p(30.0 + h, 0.0, -h), p(30.0 + h, 0.0, h),
        p(30.0 - h, 0.0, h)])?;
    let bend = make_arc(o, x, y, 30.0, 0.0, FRAC_PI_2)?;
    Ok(sweep_profile_twisted_scaled(&square, &bend, 0.0, end / h, None)?)
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let (outside, hole) = (duct(9.0, 6.0)?, duct(7.0, 4.0)?);
    Ok(boolean_operation(&outside, &hole, BooleanOperation::Subtract, &BooleanOptions::default())?)
}
