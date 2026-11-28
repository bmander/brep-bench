//! The puck of `offset/rounded-puck-shrunk-by-its-rounds`, its rims rounded to radius 2, shrunk
//! by 3: every point at least 3 from its outside, SolidCraft's offset of the whole solid inward
//! (`offset_solid`, rounded), the moved faces that cross trimmed and merged.
use super::rounded_puck_shrunk_by_its_rounds::rounded_puck;
use crate::solids::*;

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&rounded_puck()?, -3.0, true)
}
