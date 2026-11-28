//! The U-shaped plate, its outline the polygon through (0, 0), (9, 0), (9, 9), (6, 9), (6, 3),
//! (3, 3), (3, 9) and (0, 9) extruded 2 along +z, grown by 1: every point within 1 of it, its
//! convex edges rounded and the slot's two concave ones sharp, SolidCraft's offset of the whole
//! solid (`offset_solid`), its convex edges rounded.
use crate::solids::*;

pub fn build() -> R<Body> {
    let outline = polygon(&[(0.0, 0.0), (9.0, 0.0), (9.0, 9.0), (6.0, 9.0), (6.0, 3.0), (3.0, 3.0), (3.0, 9.0), (0.0, 9.0)]);
    let plate = extrude(&Plane::XY, outline, 0.0, 2.0)?;
    solidcraft_kernel::offset_solid(&plate, 1.0, true)
}
