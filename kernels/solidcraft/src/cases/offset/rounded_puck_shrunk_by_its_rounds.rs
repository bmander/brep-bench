//! A puck, the cylinder of radius 6 on the z axis from z = 0 to z = 10 with both rims rounded to
//! radius 2, shrunk by 2, the rounds' radius: every point at least 2 from its outside, SolidCraft's
//! offset of the whole solid inward (`offset_solid`, rounded), the moved faces that cross trimmed
//! and merged.
use crate::solids::*;

/// `body` with its circles about the z axis, each `(radius, z)`, rounded to `size`: each circle
/// picked at a point on it and taken whole, its tangent chain (a circle may be split at seams).
pub fn round_circles(body: &Body, circles: &[(f64, f64)], size: f64) -> R<Body> {
    let mut edges = Vec::new();
    for &(r, z) in circles {
        // picked 1 radian round from +x, clear of a seam at 0
        edges.extend(solidcraft_kernel::tangent_chain(body, &[v3(r * 1f64.cos(), r * 1f64.sin(), z)])?);
    }
    solidcraft_kernel::fillet(body, &edges, size)
}

/// The puck: the cylinder with its rims, the circles of radius 6 at z = 0 and z = 10, rounded.
pub fn rounded_puck() -> R<Body> {
    round_circles(&zcyl(0.0, 0.0, 6.0, 0.0, 10.0)?, &[(6.0, 0.0), (6.0, 10.0)], 2.0)
}

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&rounded_puck()?, -2.0, true)
}
