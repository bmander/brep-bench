//! The cylinder of radius 5 on the z axis from z = 0 to z = 10, grown by 1 to its parallel body:
//! radius 6 from z = −1 to z = 11, each rim the outer quarter of a torus's tube of radius 1.
use crate::solids::offset::parallel;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cylinder = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 10.0)?;
    Ok(parallel(&cylinder, 1.0)?)
}
