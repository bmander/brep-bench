//! The plus-shaped plate, its outline the polygon through (3, 0), (6, 0), (6, 3), (9, 3), (9, 6),
//! (6, 6), (6, 9), (3, 9), (3, 6), (0, 6), (0, 3) and (3, 3) extruded 2 along +z, grown by 1: every
//! point within 1 of it, its convex edges rounded and its four concave ones sharp, SolidCraft's
//! offset of the whole solid (`offset_solid`), its convex edges rounded.
use crate::solids::*;

/// The plus outline in the plane z = 0, extruded `thickness` along +z.
pub fn plus_plate(thickness: f64) -> R<Body> {
    let outline = polygon(&[
        (3.0, 0.0),
        (6.0, 0.0),
        (6.0, 3.0),
        (9.0, 3.0),
        (9.0, 6.0),
        (6.0, 6.0),
        (6.0, 9.0),
        (3.0, 9.0),
        (3.0, 6.0),
        (0.0, 6.0),
        (0.0, 3.0),
        (3.0, 3.0),
    ]);
    extrude(&Plane::XY, outline, 0.0, thickness)
}

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&plus_plate(2.0)?, 1.0, true)
}
