//! The box from the origin to (10, 8, 6) hollowed to a wall 1 thick with no opening: the box less
//! the box from (1, 1, 1) to (9, 7, 5), its cavity's shell facing the void.
//!
//! The kernel's `offset_shell` needs at least one opening face. With none, the hollow is the
//! kernel's own shell recipe (`offset_shell_by_boolean`): the solid less its offset body, every
//! face moved 1 inward by `offset_solid`.
use crate::solids::p;
use crate::solids::shell::less;
use crate::Refusal;
use brep_kernel::{make_box_brep, offset_solid, BrepSolid, OffsetCorners};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 8.0, 6.0)?;
    let cavity = offset_solid(&block, -1.0, OffsetCorners::Sharp)?.solid;
    less(&block, &cavity)
}
