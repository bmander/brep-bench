//! The plus-shaped plate of `offset/plus-shaped-plate-grown`, 4 thick, shrunk by 1: every point at
//! least 1 from its outside, its convex edges sharp and its four concave vertical edges quarter
//! cylinders, SolidCraft's offset of the whole solid inward (`offset_solid`), rounded where the
//! moved faces part.
use super::plus_shaped_plate_grown::plus_plate;
use crate::solids::*;

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&plus_plate(4.0)?, -1.0, true)
}
