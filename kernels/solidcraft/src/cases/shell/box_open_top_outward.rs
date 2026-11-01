//! SolidCraft's Shell (`shell`) hollows inward only. Outward, the wall is the box with the five faces
//! other than the top moved 1 outward by Offset Face (`offset_faces`), which keeps every edge
//! where its two faces, moved, meet: the box from (−1, −1, −1) to (11, 9, 6). Its eight
//! edges below the rim, rounded to radius 1, are the quarter cylinders of radius 1 about the box's
//! own edges, each 1 from the faces either side, and where three meet at a bottom corner their
//! corner is the ball of radius 1 about the box's corner: the box grown by 1 and cut off at z = 6, less the box.
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
    // a point on each edge of the grown box below the rim: four round its bottom, four up its
    // sides
    let edges = [
        v3(5.0, -1.0, -1.0),
        v3(5.0, 9.0, -1.0),
        v3(-1.0, 4.0, -1.0),
        v3(11.0, 4.0, -1.0),
        v3(-1.0, -1.0, 3.0),
        v3(11.0, -1.0, 3.0),
        v3(11.0, 9.0, 3.0),
        v3(-1.0, 9.0, 3.0),
    ];
    let rounded = solidcraft_kernel::fillet(&grown, &edges, 1.0)?;
    cut(&rounded, &b)
}
