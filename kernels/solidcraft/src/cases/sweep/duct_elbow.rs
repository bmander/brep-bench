use crate::solids::*;
use solidcraft_kernel::{PathSeg, sweep};
use std::f64::consts::FRAC_PI_2;

/// The hollow square in the xz plane at x = 30, swept a quarter turn round the arc its centre
/// follows about the z axis: the sweep carries the hole along with the outline.
pub fn build() -> R<Body> {
    let mut section = rect(21.0, -9.0, 39.0, 9.0);
    section.holes.push(rect(23.0, -7.0, 37.0, 7.0).outer.reversed());
    let arc =
        PathSeg::Arc { a: v3(30.0, 0.0, 0.0), center: Vec3::ZERO, axis: Vec3::Z, angle: FRAC_PI_2 };
    sweep(&xz(), &section, &[arc])
}
