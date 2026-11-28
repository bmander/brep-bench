//! The elliptic cylinder of `geometry/elliptic-cylinder` (x²/25 + y²/9 ≤ 1 from z = 0 to z = 10)
//! grown by 1: every point within 1 of it, its rims tubes about the ellipses, SolidCraft's offset
//! of the whole solid (`offset_solid`), its convex edges rounded.
use crate::solids::*;

/// The ellipse x²/a² + y²/b² ≤ 1, drawn as its four rational quadratic quarters
/// (`chamfer/elliptic-cylinder-rim`'s), extruded from z = 0 to z = 10.
pub fn elliptic_cylinder(a: f64, b: f64) -> R<Body> {
    let region = Region2 { outer: crate::cases::chamfer::elliptic_cylinder_rim::ellipse(a, b), holes: vec![] };
    extrude(&Plane::XY, region, 0.0, 10.0)
}

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&elliptic_cylinder(5.0, 3.0)?, 1.0, true)
}
