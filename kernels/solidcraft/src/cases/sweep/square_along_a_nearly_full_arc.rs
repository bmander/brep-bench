use crate::solids::*;
use solidcraft_kernel::{PathSeg, sweep};

/// The square from (9, 0, −1) to (11, 0, 1) in the xz plane, square to the arc where it starts,
/// swept 350° along the arc of radius 10 about the z axis; the kernel's sweep turns it along the
/// arc, so its ends are flat.
pub fn build() -> R<Body> {
    let arc = PathSeg::Arc {
        a: v3(10.0, 0.0, 0.0),
        center: Vec3::ZERO,
        axis: Vec3::Z,
        angle: 350f64.to_radians(),
    };
    sweep(&xz(), &rect(9.0, -1.0, 11.0, 1.0), &[arc])
}
