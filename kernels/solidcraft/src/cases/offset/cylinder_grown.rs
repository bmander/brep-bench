//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, grown by 1: every point within 1 of
//! it, its rims the outer quarters of tori, SolidCraft's offset of the whole solid
//! (`offset_solid`), its convex edges rounded.
use crate::solids::*;

pub fn build() -> R<Body> {
    let drum = zcyl(0.0, 0.0, 5.0, 0.0, 10.0)?;
    solidcraft_kernel::offset_solid(&drum, 1.0, true)
}
