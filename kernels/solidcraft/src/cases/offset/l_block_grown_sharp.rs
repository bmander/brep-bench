//! Offset Face (`offset_faces`) moves the faces picked along their outward normals and extends
//! them to meet, keeping the body's topology: with every face of the plate picked, that is the
//! plate grown with its edges kept sharp.
use crate::solids::*;

/// The L plate: its outline in the xy plane extruded from z = 0 to z = 8.
pub fn l_plate() -> R<Body> {
    let outline =
        polygon(&[(0.0, 0.0), (20.0, 0.0), (20.0, 5.0), (5.0, 5.0), (5.0, 15.0), (0.0, 15.0)]);
    extrude(&Plane::XY, outline, 0.0, 8.0)
}

/// A point on each of the plate's eight faces: its bottom and top, then its sides in the
/// outline's order.
pub fn l_plate_faces() -> [Vec3; 8] {
    [
        v3(2.0, 2.0, 0.0),
        v3(2.0, 2.0, 8.0),
        v3(10.0, 0.0, 4.0),
        v3(20.0, 2.5, 4.0),
        v3(12.0, 5.0, 4.0),
        v3(5.0, 10.0, 4.0),
        v3(2.5, 15.0, 4.0),
        v3(0.0, 7.0, 4.0),
    ]
}

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_faces(&l_plate()?, &l_plate_faces(), 1.0)
}
