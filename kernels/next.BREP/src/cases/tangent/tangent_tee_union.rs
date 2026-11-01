//! Two cylinders crossing at right angles: the main, of radius 5 along the x axis from x = −15 to
//! 15, and the branch, of radius 3 parallel to the y axis from y = −15 to 15 on the line x = 0,
//! z = 2. Their tops touch at (0, 0, 5); united.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let main = make_cylinder_brep(p(-15.0, 0.0, 0.0), p(1.0, 0.0, 0.0), 5.0, 30.0)?;
    let branch = make_cylinder_brep(p(0.0, -15.0, 2.0), p(0.0, 1.0, 0.0), 3.0, 30.0)?;
    Ok(union(&main, &branch)?)
}
