//! The sheet of five faces of the cube from the origin to (10, 10, 10), its floor and four walls
//! with no top, thickened by 1 into the cube, each moved face trimmed where it meets the others:
//! the cube less the box from (1, 1, 1) to (9, 9, 10).
//!
//! The kernel's thicken of a sheet of several faces (`thicken_sheet_faces`): the five faces, each
//! thickened by 1 into the cube, and the slabs united, which trims each moved face where it meets
//! the others.
use crate::solids::offset::tray;
use crate::Refusal;
use brep_kernel::{thicken_sheet_faces, BrepSolid, OffsetCorners};

pub fn build() -> Result<BrepSolid, Refusal> {
    Ok(thicken_sheet_faces(&tray()?, 1.0, OffsetCorners::Sharp)?)
}
