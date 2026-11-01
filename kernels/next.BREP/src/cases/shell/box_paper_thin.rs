//! The box from the origin to (10, 8, 6) shelled inward to a wall 0.001 thick, open at its top,
//! z = 6: the box less the box from (0.001, 0.001, 0.001) to (9.999, 7.999, 6).
use crate::solids::p;
use crate::solids::shell::shell;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 8.0, 6.0)?;
    let top = p(5.0, 4.0, 6.0);
    shell(&block, &[top], 0.001)
}
