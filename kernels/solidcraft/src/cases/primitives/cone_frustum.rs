use crate::solids::*;

/// The kernel has no cone: SolidCraft's own cone feature revolves the half section, a trapezoid
/// with an edge on the axis, and so does this.
pub fn build() -> R<Body> {
    revolve_z(polygon(&[(0.0, 0.0), (5.0, 0.0), (2.0, 6.0), (0.0, 6.0)]), 360.0)
}
