//! A plate, the cylinder of radius 8 on the z axis from z = −2 to z = 0, joined to the ball of
//! radius 3 about (0, 0, 1), the concave edge where they meet filleted to radius 1, grown by 2:
//! every point within 2 of it, SolidCraft's offset of the whole solid (`offset_solid`), its convex
//! edges rounded.
use super::rounded_puck_shrunk_by_its_rounds::round_circles;
use crate::solids::*;

/// The plate and ball joined, the circle of radius √8 at z = 0 where the ball leaves the plate's
/// top filleted to `radius`.
pub fn filleted_ball_on_plate(radius: f64) -> R<Body> {
    let plate = zcyl(0.0, 0.0, 8.0, -2.0, 0.0)?;
    let ball = solidcraft_kernel::sphere(v3(0.0, 0.0, 1.0), 3.0)?;
    round_circles(&union(&plate, &ball)?, &[(8f64.sqrt(), 0.0)], radius)
}

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&filleted_ball_on_plate(1.0)?, 2.0, true)
}
