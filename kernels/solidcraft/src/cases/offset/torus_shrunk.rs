//! A torus's one face, moved 1.5 inward along its normal by Offset Face (`offset_faces`), is the
//! torus about the same middle circle with a tube 1.5 thinner: the torus has no edge to join, so
//! moving its face is the whole of shrinking it.
use crate::solids::*;

pub fn build() -> R<Body> {
    let torus = solidcraft_kernel::torus(Vec3::ZERO, 10.0, 3.0)?;
    // the tube's outermost point, picking its face
    solidcraft_kernel::offset_faces(&torus, &[v3(13.0, 0.0, 0.0)], -1.5)
}
