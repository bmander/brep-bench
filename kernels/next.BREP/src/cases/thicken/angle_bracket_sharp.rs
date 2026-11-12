//! The sheet of two faces meeting at a right angle along a crease, the floor from (0, 0, 0) to
//! (10, 8, 0) and the wall from (0, 0, 0) to (0, 8, 10), thickened by 1 to the outside of the
//! crease (−z and −x), the moved faces extended until they meet: an angle 1 thick, sharp.
//!
//! The kernel's thicken of a sheet of several faces (`thicken_sheet_faces`), 1 along the faces'
//! normals, its joins sharp: the gap the two slabs leave at the crease is filled out to the line
//! the two moved planes share.
use crate::solids::offset::bracket;
use crate::Refusal;
use brep_kernel::{thicken_sheet_faces, BrepSolid, OffsetCorners};

pub fn build() -> Result<BrepSolid, Refusal> {
    Ok(thicken_sheet_faces(&bracket()?, 1.0, OffsetCorners::Sharp)?)
}
