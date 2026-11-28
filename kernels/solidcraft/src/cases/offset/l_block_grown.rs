//! The L-shaped plate (the polygon through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15),
//! extruded 8 along +z) grown by 1: every point within 1 of it, its convex edges rounded and its
//! concave one sharp, SolidCraft's offset of the whole solid (`offset_solid`), its convex edges
//! rounded.
use crate::solids::*;

/// The L-shaped plate.
pub fn l_block() -> R<Body> {
    let outline = polygon(&[(0.0, 0.0), (20.0, 0.0), (20.0, 5.0), (5.0, 5.0), (5.0, 15.0), (0.0, 15.0)]);
    extrude(&Plane::XY, outline, 0.0, 8.0)
}

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&l_block()?, 1.0, true)
}
