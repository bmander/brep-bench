//! The box from the origin to (10, 8, 6) shelled inward to a wall 1 thick, open at its top, z = 6,
//! and its front, y = 0, two faces sharing an edge: the box less the box from (1, 0, 1) to
//! (9, 7, 6).
use crate::solids::p;
use crate::solids::shell::shell;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 8.0, 6.0)?;
    let (top, front) = (p(5.0, 4.0, 6.0), p(5.0, 0.0, 3.0));
    shell(&block, &[top, front], 1.0)
}
