//! The sheet of five faces of the cube from the origin to (10, 10, 10), its floor and four walls
//! with no top, thickened by 1 out of the cube, the moved faces extended until they meet: the box
//! from (−1, −1, −1) to (11, 11, 10) less the cube.
//!
//! The kernel's thicken of a sheet of several faces (`thicken_sheet_faces`), 1 out of the cube,
//! its joins sharp: the gaps the slabs leave at the eight creases and the four corners are filled
//! out to where the moved planes meet.
use crate::solids::offset::tray;
use crate::Refusal;
use brep_kernel::{thicken_sheet_faces, BrepSolid, OffsetCorners};

pub fn build() -> Result<BrepSolid, Refusal> {
    Ok(thicken_sheet_faces(&tray()?, -1.0, OffsetCorners::Sharp)?)
}
