//! A barrel, the ball of radius 5 about the origin cut by the planes z = −3 and z = 3, shrunk by 1.
//! Its two edges, where the sphere meets the flat ends, are convex, and shrunk a convex edge stays
//! sharp, so Offset Face (`offset_faces`) moving the sphere and both ends 1 inward is the barrel
//! shrunk: the ball of radius 4 cut by z = ±2.
use crate::solids::*;

pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    let barrel = common(&ball, &block((-6.0, -6.0, -3.0), (6.0, 6.0, 3.0))?)?;
    // a point on the sphere's zone and the middle of each end, picking them
    let faces = [v3(5.0, 0.0, 0.0), v3(0.0, 0.0, -3.0), v3(0.0, 0.0, 3.0)];
    solidcraft_kernel::offset_faces(&barrel, &faces, -1.0)
}
