//! The L-shaped plate (the polygon through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15),
//! extruded 8 along +z) shrunk by 1: every point at least 1 from its outside, its convex edges
//! sharp and its concave edge a concave quarter cylinder, SolidCraft's offset of the whole solid
//! inward (`offset_solid`), rounded where the moved faces part.
use super::l_block_grown::l_block;
use crate::solids::*;

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&l_block()?, -1.0, true)
}
