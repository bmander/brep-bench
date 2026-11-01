//! Two cylinders of radius 5 and length 30 crossing at right angles, along the x axis from
//! x = −15 to 15 and parallel to the y axis from y = −15 to 15 on the line x = 0, z = 9.99: the
//! cushion common to both, 0.01 thick.
use crate::solids::boolean::common;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let along_x = make_cylinder_brep(p(-15.0, 0.0, 0.0), p(1.0, 0.0, 0.0), 5.0, 30.0)?;
    let along_y = make_cylinder_brep(p(0.0, -15.0, 9.99), p(0.0, 1.0, 0.0), 5.0, 30.0)?;
    Ok(common(&along_x, &along_y)?)
}
