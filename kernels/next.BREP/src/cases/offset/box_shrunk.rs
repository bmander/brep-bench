//! The box from the origin to (10, 8, 6), shrunk by 1: every point at least 1 from its outside,
//! the box from (1, 1, 1) to (9, 7, 5).
use crate::solids::offset::parallel;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 8.0, 6.0)?;
    Ok(parallel(&block, -1.0)?)
}
