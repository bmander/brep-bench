//! The cube from (−10, −10, −10) to (10, 10, 10) with the oblate spheroid of
//! `geometry/oblate-spheroid`, (x² + y²)/36 + z²/9 ≤ 1, cut out of its middle, a closed cavity,
//! grown by 1: every point within 1 of it, SolidCraft's offset of the whole solid
//! (`offset_solid`), its convex edges rounded.
use super::oblate_spheroid_grown::oblate_spheroid;
use crate::solids::*;

pub fn build() -> R<Body> {
    let cube = block((-10.0, -10.0, -10.0), (10.0, 10.0, 10.0))?;
    let hollow = cut(&cube, &oblate_spheroid()?)?;
    solidcraft_kernel::offset_solid(&hollow, 1.0, true)
}
