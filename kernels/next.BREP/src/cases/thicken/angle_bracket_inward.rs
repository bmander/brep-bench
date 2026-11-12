//! The sheet of two faces meeting at a right angle along a crease, the floor from (0, 0, 0) to
//! (10, 8, 0) and the wall from (0, 0, 0) to (0, 8, 10), thickened by 1 to the inside of the
//! crease (+z and +x), each moved face trimmed where it meets the other: an L 1 thick.
//!
//! The kernel's thicken of a sheet of several faces (`thicken_sheet_faces`): the two faces, each
//! thickened by 1 against its normal, and the slabs united, which trims each moved face where it
//! meets the other.
use crate::solids::offset::bracket;
use crate::Refusal;
use brep_kernel::{thicken_sheet_faces, BrepSolid, OffsetCorners};

pub fn build() -> Result<BrepSolid, Refusal> {
    Ok(thicken_sheet_faces(&bracket()?, -1.0, OffsetCorners::Sharp)?)
}
