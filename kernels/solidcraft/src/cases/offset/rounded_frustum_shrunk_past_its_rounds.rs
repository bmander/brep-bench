//! The cone frustum on the z axis from radius 6 at z = 0 to radius 3 at z = 6, both rims rounded
//! to radius 1.5, shrunk by 2: every point at least 2 from its outside, SolidCraft's offset of the
//! whole solid inward (`offset_solid`, rounded), the moved faces that cross trimmed and merged.
use super::rounded_puck_shrunk_by_its_rounds::round_circles;
use crate::solids::*;

/// The frustum with the rims `rims` (each `(radius, z)`: (6, 0) the bottom, (3, 6) the top)
/// rounded to 1.5. solidcraft-kernel has no cone: the frustum is its half section turned about
/// the z axis.
pub fn frustum_rounded(rims: &[(f64, f64)]) -> R<Body> {
    let frustum = revolve_z(polygon(&[(0.0, 0.0), (6.0, 0.0), (3.0, 6.0), (0.0, 6.0)]), 360.0)?;
    round_circles(&frustum, rims, 1.5)
}

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&frustum_rounded(&[(6.0, 0.0), (3.0, 6.0)])?, -2.0, true)
}
