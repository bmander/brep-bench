//! A pipe of radius 20 swept along the planar cubic Bézier curve with control points (0, 0, 0),
//! (12, −16, 0), (32, −16, 0) and (44, 0, 0), its section the disc square to the curve, its ends
//! flat: the kernel's path sweep of the circle along the curve.
use crate::solids::p;
use crate::solids::sweep::{bezier, circle};
use crate::Refusal;
use brep_kernel::{sweep_profile_along_path, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let path = bezier(&[p(0.0, 0.0, 0.0), p(12.0, -16.0, 0.0), p(32.0, -16.0, 0.0),
        p(44.0, 0.0, 0.0)])?;
    // the section at the start, square to the curve's first direction (3, −4, 0)
    let section = circle(p(0.0, 0.0, 0.0), p(0.8, 0.6, 0.0), p(0.0, 0.0, 1.0), 20.0)?;
    Ok(sweep_profile_along_path(&section, &path, None)?)
}
