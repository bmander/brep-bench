//! A circle of radius 2 swept along the quarter arc of radius 10 about the z axis in the xy plane,
//! from (10, 0, 0) to (0, 10, 0), square to the path, its ends flat: the kernel's path sweep.
use crate::solids::p;
use crate::solids::sweep::circle;
use crate::Refusal;
use brep_kernel::{make_arc, sweep_profile_along_path, BrepSolid};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, y, z) = (p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), p(0.0, 0.0, 1.0));
    let path = make_arc(p(0.0, 0.0, 0.0), x, y, 10.0, 0.0, FRAC_PI_2)?;
    // the section at the start, in the xz plane square to the arc
    let section = circle(p(10.0, 0.0, 0.0), x, z, 2.0)?;
    Ok(sweep_profile_along_path(&section, &path, None)?)
}
