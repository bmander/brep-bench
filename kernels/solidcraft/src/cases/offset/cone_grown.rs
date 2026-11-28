//! A cone on the z axis, its base the disc of radius 3 at z = 0 and its apex at (0, 0, 4), grown
//! by 1: every point within 1 of it, its rim a piece of a torus's tube and its apex a cap of a
//! ball, SolidCraft's offset of the whole solid (`offset_solid`), its convex edges rounded.
use crate::solids::*;

pub fn build() -> R<Body> {
    // solidcraft-kernel has no cone: its half section turned about the z axis
    let cone = revolve_z(polygon(&[(0.0, 0.0), (3.0, 0.0), (0.0, 4.0)]), 360.0)?;
    solidcraft_kernel::offset_solid(&cone, 1.0, true)
}
