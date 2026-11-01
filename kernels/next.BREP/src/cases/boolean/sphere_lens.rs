//! Two balls of radius 5, about the origin and about (6, 0, 0): the lens common to both.
use crate::solids::boolean::common;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let first = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    let second = make_sphere_brep(p(6.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    Ok(common(&first, &second)?)
}
