//! SolidCraft's Shell (`shell`) wants a face to open. With none, the hollow is the solid less its
//! cavity, the solid with every face moved 0.5 inward by Offset Face (`offset_faces`).
use crate::solids::*;

pub fn build() -> R<Body> {
    let torus = solidcraft_kernel::torus(Vec3::ZERO, 6.0, 2.0)?;
    // the tube's outermost point, picking its face
    let cavity = solidcraft_kernel::offset_faces(&torus, &[v3(8.0, 0.0, 0.0)], -0.5)?;
    cut(&torus, &cavity)
}
