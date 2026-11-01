//! The box from the origin to (10, 8, 6) shelled inward to a wall 4 thick, open at its top, z = 6:
//! the walls at y = 0 and y = 8, moved 4 inward, meet in the plane y = 4, the cavity has no
//! volume, and the result is the box itself.
use crate::solids::p;
use crate::solids::shell::shell;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 8.0, 6.0)?;
    let top = p(5.0, 4.0, 6.0);
    shell(&block, &[top], 4.0)
}
