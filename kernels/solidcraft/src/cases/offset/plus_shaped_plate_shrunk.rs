//! The plus-shaped plate of `offset/plus-shaped-plate-grown`, 4 thick, shrunk by 1: every point at
//! least 1 from its outside, its convex edges sharp and its four concave vertical edges quarter
//! cylinders, the kernel's offset inward, rounded where the moved faces part.
use super::plus_shaped_plate_grown::plus_plate;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&plus_plate(4.0)?, -1.0)
}
