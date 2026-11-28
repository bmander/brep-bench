//! SolidCraft's Shell (`shell`) hollows a body inward to a wall of the thickness, opening the faces
//! picked by points on them. `shell/bored-frustum-sealed-below-its-opening` with every length 3.1
//! times as long, each written as the case gives it.
use crate::solids::*;

/// solidcraft-kernel has no cone: SolidCraft's own cone feature revolves the half section, and so does
/// this, the section a trapezoid kept off the axis by the bore's radius.
pub fn build() -> R<Body> {
    let bored = revolve_z(polygon(&[(3.1, 0.0), (18.6, 0.0), (9.3, 12.4), (3.1, 12.4)]), 360.0)?;
    // the top, the annulus between radii 3.1 and 9.3, picked at radius 6.2
    solidcraft_kernel::shell(&bored, &[v3(0.0, 6.2, 12.4)], 3.1)
}
