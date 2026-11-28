//! The L-shaped plate of `offset/l-block-grown` (the polygon through (0, 0), (20, 0), (20, 5),
//! (5, 5), (5, 15) and (0, 15), extruded 8 along +z) with its concave edge, along (5, 5), rounded
//! to radius 4, grown by 5, more than the round's radius: every point within 5 of it, SolidCraft's
//! offset of the whole solid (`offset_solid`), its convex edges rounded.
use super::l_block_grown::l_block;
use crate::solids::*;

pub fn build() -> R<Body> {
    // the inside edge, picked at its middle
    let plate = solidcraft_kernel::fillet(&l_block()?, &[v3(5.0, 5.0, 4.0)], 4.0)?;
    solidcraft_kernel::offset_solid(&plate, 5.0, true)
}
