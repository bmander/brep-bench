//! A drum, the cylinder of radius 8 on the z axis from z = 0 to z = 6, with a flange round its
//! side out to radius 11 from z = 2.2 to z = 3.8, shrunk by 1: every point at least 1 from its
//! outside. The flange's faces, moved 1 in from each side, cross and are trimmed away by
//! SolidCraft's offset of the whole solid inward (`offset_solid`, rounded; its self-intersections
//! merged), and the flange's inside edges, where the moved faces part, become concave tori.
use crate::solids::*;

pub fn build() -> R<Body> {
    let drum = zcyl(0.0, 0.0, 8.0, 0.0, 6.0)?;
    let flange = zcyl(0.0, 0.0, 11.0, 2.2, 3.8)?;
    solidcraft_kernel::offset_solid(&union(&drum, &flange)?, -1.0, true)
}
