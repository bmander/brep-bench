//! A cylinder of radius 3 about the z axis from z = −10 up to the plane z = x·tan 30°, grown by 1:
//! every point within 1 of it, SolidCraft's offset of the whole solid (`offset_solid`), its convex
//! edges rounded. The cylinder is that of `offset/slant-cut-cylinder-grown`, cut at 30° instead of
//! 45°.
use super::slant_cut_cylinder_grown::slant_cut_cylinder;
use crate::solids::*;

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&slant_cut_cylinder(30.0)?, 1.0, true)
}
