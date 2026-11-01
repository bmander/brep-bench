//! The box from the origin to (10, 8, 6) shelled outward to a wall 1 thick, open at its top, z = 6,
//! its edges kept sharp: the box from (-1, -1, -1) to (11, 9, 6) less the box. The kernel shells
//! outward for a negative thickness.
use crate::solids::p;
use crate::solids::shell::shell;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 8.0, 6.0)?;
    let top = p(5.0, 4.0, 6.0);
    shell(&block, &[top], -1.0)
}
