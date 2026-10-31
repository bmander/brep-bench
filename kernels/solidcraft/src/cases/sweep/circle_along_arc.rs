use crate::solids::*;
use solidcraft_kernel::{PathSeg, sweep};
use std::f64::consts::FRAC_PI_2;

/// The circle stands square to the arc where it starts, in the xz plane about (10, 0, 0); the
/// kernel's sweep turns it along the arc, so its ends are flat.
pub fn build() -> R<Body> {
    let arc =
        PathSeg::Arc { a: v3(10.0, 0.0, 0.0), center: Vec3::ZERO, axis: Vec3::Z, angle: FRAC_PI_2 };
    sweep(&xz(), &disc((10.0, 0.0), 2.0), &[arc])
}
