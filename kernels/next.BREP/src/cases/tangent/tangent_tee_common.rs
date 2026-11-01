//! Two cylinders crossing at right angles: of radius 5 along the x axis from x = −15 to 15, and of
//! radius 3 parallel to the y axis from y = −15 to 15 on the line x = 0, z = 2. Their tops touch
//! at (0, 0, 5); the solid common to both.
use crate::solids::boolean::common;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let first = make_cylinder_brep(p(-15.0, 0.0, 0.0), p(1.0, 0.0, 0.0), 5.0, 30.0)?;
    let second = make_cylinder_brep(p(0.0, -15.0, 2.0), p(0.0, 1.0, 0.0), 3.0, 30.0)?;
    Ok(common(&first, &second)?)
}
