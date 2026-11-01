//! The bent sheet: the profile along the line from (−10, 0, 0) to (0, 0, 0), round the quarter
//! circle of radius 2 about (0, 0, 2) to (2, 0, 2) and up the line to (2, 0, 12), swept along +y
//! from y = 0 to 8, three faces meeting tangentially, thickened by 1 to the outside of the bend:
//! a bracket 1 thick, its bend's radii 2 and 3.
//!
//! next.BREP thickens one face at a time, so the sheet's three faces are thickened as faces of a
//! solid, the profile closed behind the bend and extruded, by the kernel's shell grown outward:
//! the solid's four other faces are its openings.
use crate::solids::offset::{flange, shell};
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    let (body, openings) = flange()?;
    shell(&body, &openings, -1.0)
}
