//! The kernel's offset of a solid is Offset Face (`offset_faces`): the torus's one face moved 3
//! outward along its normal, to the torus of tube 5 about the same middle circle, of radius 4.
//! That torus crosses itself through the axis, and the grown solid is its outer part, the apple;
//! an offset that keeps the topology has no face to trim it to, and the kernel's error, if it
//! declines, is the refusal.
use crate::solids::*;

pub fn build() -> R<Body> {
    let torus = solidcraft_kernel::torus(Vec3::ZERO, 4.0, 2.0)?;
    // the tube's outermost point, picking its face
    solidcraft_kernel::offset_faces(&torus, &[v3(6.0, 0.0, 0.0)], 3.0)
}
