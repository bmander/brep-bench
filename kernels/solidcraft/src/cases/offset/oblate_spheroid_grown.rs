//! The oblate spheroid of `geometry/oblate-spheroid`, (x² + y²)/36 + z²/9 ≤ 1, grown by 1: every
//! point within 1 of it, bounded by its one face moved 1 out along its normal. The spheroid has no
//! edge to round, and this is SolidCraft's offset of the whole solid (`offset_solid`), rounded.
use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::FRAC_1_SQRT_2;

/// The spheroid: the half ellipse x²/36 + z²/9 = 1, x ≥ 0, in x and z, closed along the z axis
/// and turned a whole turn about it. The half ellipse is the two rational quadratic quarters it
/// is, through (0, −3), (6, 0) and (0, 3), the corners of the rectangle about them weighted √2/2.
pub fn oblate_spheroid() -> R<Body> {
    let (bottom, waist, top) = (v2(0.0, -3.0), v2(6.0, 0.0), v2(0.0, 3.0));
    let lower = Seg2::Conic { a: bottom, apex: v2(6.0, -3.0), b: waist, w: FRAC_1_SQRT_2 };
    let upper = Seg2::Conic { a: waist, apex: v2(6.0, 3.0), b: top, w: FRAC_1_SQRT_2 };
    let outer = Loop2 { segs: vec![lower, upper, Seg2::Line { a: top, b: bottom }] };
    revolve_z(Region2 { outer, holes: vec![] }, 360.0)
}

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&oblate_spheroid()?, 1.0, true)
}
