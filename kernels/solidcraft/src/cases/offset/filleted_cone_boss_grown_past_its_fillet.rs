//! The cone boss on a plate of `offset/filleted-cone-boss-grown-by-its-fillet`, its concave edge
//! filleted to radius 1, grown by 2: every point within 2 of it, SolidCraft's offset of the whole
//! solid (`offset_solid`), its convex edges rounded.
use super::filleted_cone_boss_grown_by_its_fillet::filleted_cone_boss;
use crate::solids::*;

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&filleted_cone_boss(1.0)?, 2.0, true)
}
