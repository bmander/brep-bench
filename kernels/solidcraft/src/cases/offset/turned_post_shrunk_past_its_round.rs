//! A turned post on the z axis, the cylinder of radius 5 from z = 0 to z = 6 topped by the frustum
//! to radius 2 at z = 10, the edge between them rounded to radius 1, shrunk by 1.5: every point at
//! least 1.5 from its outside, SolidCraft's offset of the whole solid inward (`offset_solid`,
//! rounded), the moved faces that cross trimmed and merged.
use super::rounded_puck_shrunk_by_its_rounds::round_circles;
use crate::solids::*;

/// The post, its half-section in x and z turned about the z axis, then the circle of radius 5 at
/// z = 6 rounded.
pub fn turned_post() -> R<Body> {
    let post = revolve_z(polygon(&[(0.0, 0.0), (5.0, 0.0), (5.0, 6.0), (2.0, 10.0), (0.0, 10.0)]), 360.0)?;
    round_circles(&post, &[(5.0, 6.0)], 1.0)
}

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&turned_post()?, -1.5, true)
}
