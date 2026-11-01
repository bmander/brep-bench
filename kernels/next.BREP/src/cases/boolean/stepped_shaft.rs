//! A stepped shaft on the z axis, one solid: a cylinder of radius 5 from z = 0 to 10, and one of
//! radius 3 from z = 10 to 20 standing on it.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let lower = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 10.0)?;
    let upper = make_cylinder_brep(p(0.0, 0.0, 10.0), p(0.0, 0.0, 1.0), 3.0, 10.0)?;
    Ok(union(&lower, &upper)?)
}
