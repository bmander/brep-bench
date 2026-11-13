//! An L-shaped block shelled outward open at its step, the walls beyond the opening kept. The
//! outline through (0, 0), (20, 0), (20, 20), (15, 20), (15, 5) and (0, 5), extruded 20 along +z,
//! shelled outward to a wall 1 thick, open at the face y = 5, its edges sharp.
use crate::solids::*;

pub fn build() -> R<Body> {
    let outline = [(0.0, 0.0), (20.0, 0.0), (20.0, 20.0), (15.0, 20.0), (15.0, 5.0), (0.0, 5.0)];
    let block = extrude(&Plane::XY, polygon(&outline), 0.0, 20.0)?;
    // the middle of each face but the opening, picking it
    let faces = [
        v3(0.0, 2.5, 10.0),
        v3(10.0, 0.0, 10.0),
        v3(20.0, 10.0, 10.0),
        v3(17.5, 20.0, 10.0),
        v3(15.0, 12.5, 10.0),
        v3(17.5, 10.0, 0.0),
        v3(17.5, 10.0, 20.0),
    ];
    let grown = solidcraft_kernel::offset_faces(&block, &faces, 1.0)?;
    cut(&grown, &block)
}
