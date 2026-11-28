//! The elliptic cylinder of `geometry/elliptic-cylinder`, x²/25 + y²/9 ≤ 1 from z = 0 to z = 10,
//! shrunk by 1: every point at least 1 from its outside. Its edges are convex and stay sharp
//! shrunk, and the side's least radius of curvature, 1.8, is more than 1, so it moves in without
//! folding: Offset Face moving every face 1 inward would be the solid shrunk. SolidCraft's
//! (`offset_faces`, on the ellipse as its profile's four conics) refuses it ("how far its
//! neighbours extend to meet it has no certified bound"), and so does ops's ("its surface opens
//! across its seam"), so this is SolidCraft's offset of the whole solid inward (`offset_solid`,
//! rounded).
use super::elliptic_cylinder_grown::elliptic_cylinder;
use crate::solids::*;

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&elliptic_cylinder(5.0, 3.0)?, -1.0, true)
}
