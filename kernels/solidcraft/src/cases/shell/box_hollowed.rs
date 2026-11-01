//! SolidCraft's Shell (`shell`) wants a face to open. With none, the hollow is the solid less its
//! cavity, the solid with every face moved 1 inward by Offset Face (`offset_faces`).
use crate::solids::*;

pub fn build() -> R<Body> {
    let b = block((0.0, 0.0, 0.0), (10.0, 8.0, 6.0))?;
    // the middle of each face, picking it
    let faces = [
        v3(0.0, 4.0, 3.0),
        v3(10.0, 4.0, 3.0),
        v3(5.0, 0.0, 3.0),
        v3(5.0, 8.0, 3.0),
        v3(5.0, 4.0, 0.0),
        v3(5.0, 4.0, 6.0),
    ];
    let cavity = solidcraft_kernel::offset_faces(&b, &faces, -1.0)?;
    cut(&b, &cavity)
}
