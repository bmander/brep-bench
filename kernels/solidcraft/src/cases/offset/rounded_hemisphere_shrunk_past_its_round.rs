//! The half of the ball of radius 5 about the origin above z = 0, its rim rounded to radius 1,
//! shrunk by 3/2: every point at least 3/2 from its outside, SolidCraft's offset of the whole
//! solid inward (`offset_solid`, rounded), the moved faces that cross trimmed and merged.
use super::rounded_puck_shrunk_by_its_rounds::round_circles;
use crate::solids::*;

pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    let below = block((-6.0, -6.0, -6.0), (6.0, 6.0, 0.0))?;
    let dome = cut(&ball, &below)?;
    solidcraft_kernel::offset_solid(&round_circles(&dome, &[(5.0, 0.0)], 1.0)?, -1.5, true)
}
