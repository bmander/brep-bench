//! A plate, the cylinder of radius 8 on the z axis from z = −2 to z = 0, joined to a boss, the
//! cone frustum from radius 4 at z = 0 to radius 1 at z = 6, the concave edge where they meet
//! filleted to radius 1, grown by 1: every point within 1 of it, SolidCraft's offset of the whole
//! solid (`offset_solid`), its convex edges rounded.
use super::rounded_puck_shrunk_by_its_rounds::round_circles;
use crate::solids::*;

/// The plate and boss joined, the circle of radius 4 at z = 0 where the boss stands on the
/// plate's top filleted to `radius`.
pub fn filleted_cone_boss(radius: f64) -> R<Body> {
    let plate = zcyl(0.0, 0.0, 8.0, -2.0, 0.0)?;
    // solidcraft-kernel has no cone: the frustum's half section turned about the z axis
    let boss = revolve_z(polygon(&[(0.0, 0.0), (4.0, 0.0), (1.0, 6.0), (0.0, 6.0)]), 360.0)?;
    round_circles(&union(&plate, &boss)?, &[(4.0, 0.0)], radius)
}

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&filleted_cone_boss(1.0)?, 1.0, true)
}
