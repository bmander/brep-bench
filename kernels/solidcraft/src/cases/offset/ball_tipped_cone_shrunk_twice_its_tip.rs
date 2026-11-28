//! The ball-tipped cone of `offset/ball-tipped-cone-shrunk-past-its-tip`, its tip a ball of radius
//! 1, shrunk by 2: every point at least 2 from its outside, SolidCraft's offset of the whole solid
//! inward (`offset_solid`, rounded), the moved faces that cross trimmed and merged.
use super::ball_tipped_cone_shrunk_past_its_tip::ball_tipped_cone;
use crate::solids::*;

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&ball_tipped_cone()?, -2.0, true)
}
