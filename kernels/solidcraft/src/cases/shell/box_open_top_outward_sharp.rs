//! SolidCraft's Shell (`shell`) hollows inward only. Outward, the wall is the box with the five faces
//! other than the top moved 1 outward by Offset Face (`offset_faces`), which keeps every edge
//! where its two faces, moved, meet, less the box.
use crate::solids::*;

pub fn build() -> R<Body> {
    let b = block((0.0, 0.0, 0.0), (10.0, 8.0, 6.0))?;
    // the middle of each face but the top, picking it
    let faces = [
        v3(0.0, 4.0, 3.0),
        v3(10.0, 4.0, 3.0),
        v3(5.0, 0.0, 3.0),
        v3(5.0, 8.0, 3.0),
        v3(5.0, 4.0, 0.0),
    ];
    let grown = solidcraft_kernel::offset_faces(&b, &faces, 1.0)?;
    cut(&grown, &b)
}
