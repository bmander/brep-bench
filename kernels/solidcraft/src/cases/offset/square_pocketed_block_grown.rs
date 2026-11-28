//! The block from (0, 0, 0) to (20, 20, 6) with the square pocket from (6, 6) to (14, 14) cut down
//! from its top to a floor at z = 3, grown by 1: every point within 1 of it, its convex edges
//! rounded and the pocket's concave ones sharp, SolidCraft's offset of the whole solid
//! (`offset_solid`), its convex edges rounded.
use crate::solids::*;

pub fn build() -> R<Body> {
    let solid = block((0.0, 0.0, 0.0), (20.0, 20.0, 6.0))?;
    // the pocket's cutter runs on past the top
    let pocket = block((6.0, 6.0, 3.0), (14.0, 14.0, 7.0))?;
    solidcraft_kernel::offset_solid(&cut(&solid, &pocket)?, 1.0, true)
}
