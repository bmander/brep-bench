//! The cylinder of `offset/slant-cut-cylinder-grown`, radius 3 about the z axis from z = −10 up to
//! the plane z = x·tan 45°, grown by 4: every point within 4 of it, SolidCraft's offset of
//! the whole solid (`offset_solid`), its convex edges rounded.
use super::slant_cut_cylinder_grown::slant_cut_cylinder;
use crate::solids::*;

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&slant_cut_cylinder(45.0)?, 4.0, true)
}
