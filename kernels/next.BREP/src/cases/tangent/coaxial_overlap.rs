//! Two cylinders of radius 5 on the z axis, from z = 0 to 10 and from z = 6 to 16, united: their
//! walls coincide between z = 6 and 10, and the union is one cylinder from z = 0 to 16. The
//! kernel's cylinders are seamed in the same place, so the second is turned a quarter turn about
//! the z axis.
use crate::solids::boolean::{turned_about_z, union};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    let first = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 10.0)?;
    let second = make_cylinder_brep(p(0.0, 0.0, 6.0), p(0.0, 0.0, 1.0), 5.0, 10.0)?;
    let second = turned_about_z(&second, FRAC_PI_2)?;
    Ok(union(&first, &second)?)
}
