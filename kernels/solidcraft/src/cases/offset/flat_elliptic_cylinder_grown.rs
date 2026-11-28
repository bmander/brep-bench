//! The elliptic cylinder x²/100 + y²/4 ≤ 1 from z = 0 to z = 10 grown by 1: every point within 1
//! of it, its rims tubes about the ellipses, SolidCraft's offset of the whole solid
//! (`offset_solid`), its convex edges rounded.
use super::elliptic_cylinder_grown::elliptic_cylinder;
use crate::solids::*;

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&elliptic_cylinder(10.0, 2.0)?, 1.0, true)
}
