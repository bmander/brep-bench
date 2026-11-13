//! The bent sheet: the profile along the line from (−10, 0, 0) to (0, 0, 0), round the quarter
//! circle of radius 2 about (0, 0, 2) to (2, 0, 2) and up the line to (2, 0, 12), swept along +y
//! from y = 0 to 8, three faces meeting tangentially, thickened by 1 to the outside of the bend:
//! a bracket 1 thick, its bend's radii 2 and 3.
//!
//! The kernel's thicken of a sheet of several faces (`thicken_sheet_faces`): the three faces, each
//! thickened by 1 away from the bend's axis, and the slabs united; the faces meet smoothly, so no
//! join is filled.
use crate::solids::offset::flange;
use crate::Refusal;
use brep_kernel::{thicken_sheet_faces, BrepSolid, OffsetCorners};

pub fn build() -> Result<BrepSolid, Refusal> {
    Ok(thicken_sheet_faces(&flange()?, -1.0, OffsetCorners::Sharp)?)
}
