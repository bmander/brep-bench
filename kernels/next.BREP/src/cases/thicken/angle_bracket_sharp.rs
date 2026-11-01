//! The sheet of two faces meeting at a right angle along a crease, the floor from (0, 0, 0) to
//! (10, 8, 0) and the wall from (0, 0, 0) to (0, 8, 10), thickened by 1 to the outside of the
//! crease (−z and −x), the moved faces extended until they meet: an angle 1 thick, sharp.
//!
//! next.BREP thickens one face at a time, so a sheet of two faces is thickened as two faces of a
//! solid, the box they bound, by the kernel's shell grown outward: the box's four other faces are
//! its openings.
use crate::solids::offset::{bracket, shell};
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    let (block, openings) = bracket()?;
    shell(&block, &openings, -1.0)
}
