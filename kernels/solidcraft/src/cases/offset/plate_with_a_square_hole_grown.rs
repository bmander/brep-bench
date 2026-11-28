//! The plate from (0, 0, 0) to (10, 10, 2) with the square hole from (3, 3) to (7, 7) through it,
//! grown by 1: every point within 1 of it, its convex edges rounded and the hole's four concave
//! corners sharp, SolidCraft's offset of the whole solid (`offset_solid`), its convex edges
//! rounded.
use crate::solids::*;

pub fn build() -> R<Body> {
    let plate = block((0.0, 0.0, 0.0), (10.0, 10.0, 2.0))?;
    // the hole's cutter runs past both faces
    let hole = block((3.0, 3.0, -1.0), (7.0, 7.0, 3.0))?;
    solidcraft_kernel::offset_solid(&cut(&plate, &hole)?, 1.0, true)
}
