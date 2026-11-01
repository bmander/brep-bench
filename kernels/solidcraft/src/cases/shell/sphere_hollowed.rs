//! SolidCraft's Shell (`shell`) wants a face to open. With none, the hollow is the solid less its
//! cavity, the solid with every face moved 1 inward by Offset Face (`offset_faces`).
use crate::solids::*;

pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    let cavity = solidcraft_kernel::offset_faces(&ball, &[v3(0.0, 0.0, 5.0)], -1.0)?;
    cut(&ball, &cavity)
}
