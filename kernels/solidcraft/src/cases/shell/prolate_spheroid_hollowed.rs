//! SolidCraft's Shell (`shell`) with no face to open: the prolate spheroid hollowed to a wall 1
//! thick round a closed cavity.
use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::FRAC_1_SQRT_2;

pub fn build() -> R<Body> {
    solidcraft_kernel::shell(&spheroid(3.0, 6.0)?, &[], 1.0)
}

/// The spheroid (x² + y²)/a² + z²/c² ≤ 1: the half ellipse x²/a² + z²/c² = 1, x ≥ 0, of the xz
/// plane, closed along the z axis, turned a whole turn about it. The half ellipse is the rational
/// quadratic it is: two quarters, the corners of the rectangle about them weighted √2/2.
pub fn spheroid(a: f64, c: f64) -> R<Body> {
    let quarter = |p: (f64, f64), apex: (f64, f64), q: (f64, f64)| Seg2::Conic {
        a: v2(p.0, p.1),
        apex: v2(apex.0, apex.1),
        b: v2(q.0, q.1),
        w: FRAC_1_SQRT_2,
    };
    let half = Loop2 {
        segs: vec![
            quarter((0.0, -c), (a, -c), (a, 0.0)),
            quarter((a, 0.0), (a, c), (0.0, c)),
            Seg2::Line { a: v2(0.0, c), b: v2(0.0, -c) },
        ],
    };
    revolve_z(Region2 { outer: half, holes: vec![] }, 360.0)
}
