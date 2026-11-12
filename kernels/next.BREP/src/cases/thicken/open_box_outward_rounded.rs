//! The sheet of five faces of the cube from the origin to (10, 10, 10), its floor and four walls
//! with no top, thickened by 1 out of the cube, the gaps the moved faces leave closed by the
//! quarter cylinders of radius 1 about the tray's eight edges and the eighth balls of radius 1
//! about the floor's four corners.
//!
//! The kernel's thicken of a sheet of several faces (`thicken_sheet_faces`), 1 out of the cube,
//! its joins rounded: the gaps the slabs leave are closed by the quarter cylinders of radius 1
//! about the eight creases and the eighth balls of radius 1 about the four corners.
use crate::solids::offset::tray;
use crate::Refusal;
use brep_kernel::{thicken_sheet_faces, BrepSolid, OffsetCorners};

pub fn build() -> Result<BrepSolid, Refusal> {
    Ok(thicken_sheet_faces(&tray()?, -1.0, OffsetCorners::Rounded)?)
}
