//! The elliptic cylinder of `geometry/elliptic-cylinder` (x²/25 + y²/9 ≤ 1 from z = 0 to z = 10)
//! shrunk by 2.5, more than the ellipse's least radius of curvature (1.8): the moved side folds
//! into swallowtails at the ends of the long axis, which SolidCraft's offset of the whole solid
//! inward (`offset_solid`, rounded) trims away where the moved side crosses itself (its
//! self-intersections merged).
use super::elliptic_cylinder_grown::elliptic_cylinder;
use crate::solids::*;

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&elliptic_cylinder(5.0, 3.0)?, -2.5, true)
}
