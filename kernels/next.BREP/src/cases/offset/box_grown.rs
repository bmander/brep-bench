//! The box from the origin to (10, 8, 6), grown by 1 to its parallel body: every point within 1 of
//! it, its edges quarter cylinders and its corners eighth balls of radius 1.
use crate::solids::offset::parallel;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 8.0, 6.0)?;
    Ok(parallel(&block, 1.0)?)
}
