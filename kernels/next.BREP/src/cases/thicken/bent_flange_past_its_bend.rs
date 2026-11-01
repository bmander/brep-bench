//! The bent sheet of `thicken/bent-flange` thickened by 3 to the inside of the bend, toward its
//! axis, more than the bend's radius 2: every point p + s·n, s from 0 to 3. The bend's moved face
//! turns inside out and is no part of the solid; the moved floor and wall meet in a concave edge
//! along (−1, y, 3).
//!
//! next.BREP thickens one face at a time, so the sheet's three faces are thickened as faces of a
//! solid, the profile closed behind the bend and extruded, by the kernel's shell into it: the
//! solid's four other faces are its openings.
use crate::solids::offset::{flange, shell};
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    let (body, openings) = flange()?;
    shell(&body, &openings, 3.0)
}
