//! The block from (−10, −10, 0) to (10, 10, 10) with a blind bore of radius 3 down the z axis from
//! its top to a flat floor at z = 2, grown by 4, more than the bore's radius, to its parallel body:
//! the bore fills, and its mouth leaves a dimple in the grown top.
//!
//! The bore is cut by a cylinder from its floor up past the top.
use crate::solids::offset::{parallel, subtract};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(-10.0, -10.0, 0.0), 20.0, 20.0, 10.0)?;
    let bore = make_cylinder_brep(p(0.0, 0.0, 2.0), p(0.0, 0.0, 1.0), 3.0, 9.0)?;
    let bored = subtract(&block, &bore)?;
    Ok(parallel(&bored, 4.0)?)
}
