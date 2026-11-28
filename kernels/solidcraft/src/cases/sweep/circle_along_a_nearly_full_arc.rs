use crate::solids::*;
use solidcraft_kernel::{PathSeg, sweep};

/// `sweep/circle-along-arc` carried round 350°: the circle stands square to the arc where it
/// starts, in the xz plane about (10, 0, 0); the kernel's sweep turns it along the arc, so its
/// ends are flat.
pub fn build() -> R<Body> {
    let arc = PathSeg::Arc {
        a: v3(10.0, 0.0, 0.0),
        center: Vec3::ZERO,
        axis: Vec3::Z,
        angle: 350f64.to_radians(),
    };
    sweep(&xz(), &disc((10.0, 0.0), 2.0), &[arc])
}
