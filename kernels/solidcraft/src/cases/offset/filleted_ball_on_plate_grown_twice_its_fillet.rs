//! The ball on a plate of `offset/filleted-ball-on-plate-grown-past-its-fillet`, its concave edge
//! filleted to radius 1.5, grown by 3: every point within 3 of it, SolidCraft's offset of the
//! whole solid (`offset_solid`), its convex edges rounded.
use super::filleted_ball_on_plate_grown_past_its_fillet::filleted_ball_on_plate;
use crate::solids::*;

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&filleted_ball_on_plate(1.5)?, 3.0, true)
}
