//! SolidCraft's Offset Face (`offset_faces`) moves the faces picked along their outward normals,
//! keeping the body's topology: each edge goes to where its two faces, moved, meet. Every edge of
//! a box is convex, and shrunk a convex edge stays sharp, so the box with all six faces moved 1
//! inward is the box shrunk by 1.
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
    solidcraft_kernel::offset_faces(&b, &faces, -1.0)
}
