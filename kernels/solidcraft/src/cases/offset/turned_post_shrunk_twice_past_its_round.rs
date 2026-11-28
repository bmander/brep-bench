//! The turned post of `offset/turned-post-shrunk-past-its-round`, its shoulder rounded to radius
//! 1, shrunk by 2: every point at least 2 from its outside, SolidCraft's offset of the whole solid
//! inward (`offset_solid`, rounded), the moved faces that cross trimmed and merged.
use super::turned_post_shrunk_past_its_round::turned_post;
use crate::solids::*;

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&turned_post()?, -2.0, true)
}
