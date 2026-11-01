//! The sheet of the whole sphere of radius 5 about the origin, one closed face with no solid
//! inside it, thickened by 1 outward: the hollow ball between radii 5 and 6.
use crate::solids::offset::thicken;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_sphere_surface, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let sheet = make_sphere_surface(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    thicken(&sheet, &[], 1.0, |q| q)
}
