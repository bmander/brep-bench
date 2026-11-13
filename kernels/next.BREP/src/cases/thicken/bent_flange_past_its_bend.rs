//! The bent sheet of `thicken/bent-flange` thickened by 3 to the inside of the bend, toward its
//! axis, more than the bend's radius 2: every point p + s·n, s from 0 to 3. The bend's moved face
//! turns inside out and is no part of the solid; the moved floor and wall meet in a concave edge
//! along (−1, y, 3).
//!
//! The kernel's thicken of a sheet of several faces (`thicken_sheet_faces`): the three faces, each
//! thickened by 3 toward the bend's axis, and the slabs united.
use crate::solids::offset::flange;
use crate::Refusal;
use brep_kernel::{thicken_sheet_faces, BrepSolid, OffsetCorners};

pub fn build() -> Result<BrepSolid, Refusal> {
    Ok(thicken_sheet_faces(&flange()?, 3.0, OffsetCorners::Sharp)?)
}
