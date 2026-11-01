//! SolidCraft's Shell (`shell`) hollows a body inward to a wall of the thickness, opening the faces
//! picked by points on them.
use crate::solids::*;

/// The kernel has no cone: SolidCraft's own cone feature revolves the half section, here a right
/// triangle with an edge on the axis, and so does this.
pub fn build() -> R<Body> {
    let cone = revolve_z(polygon(&[(0.0, 0.0), (6.0, 0.0), (0.0, 8.0)]), 360.0)?;
    solidcraft_kernel::shell(&cone, &[v3(1.0, 1.0, 0.0)], 1.0)
}
