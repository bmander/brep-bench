//! The box from the origin to (10, 8, 6) shelled inward to a wall 1 thick, open at its top, z = 6,
//! and its bottom, z = 0: a rectangular tube, the box less the box from (1, 1, 0) to (9, 7, 6).
use crate::solids::p;
use crate::solids::shell::shell;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 8.0, 6.0)?;
    let (top, bottom) = (p(5.0, 4.0, 6.0), p(5.0, 4.0, 0.0));
    shell(&block, &[top, bottom], 1.0)
}
