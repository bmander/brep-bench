//! The plate from (−10, −10, 0) to (10, 10, 4) with a round recess of radius 3 sunk 1 into its top
//! about the z axis, grown by 4, more than the recess's radius, to its parallel body.
//!
//! The recess is cut by a cylinder from its floor at z = 3 up past the top.
use crate::solids::offset::{parallel, subtract};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(-10.0, -10.0, 0.0), 20.0, 20.0, 4.0)?;
    let recess = make_cylinder_brep(p(0.0, 0.0, 3.0), p(0.0, 0.0, 1.0), 3.0, 2.0)?;
    let recessed = subtract(&plate, &recess)?;
    Ok(parallel(&recessed, 4.0)?)
}
