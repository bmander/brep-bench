//! SolidCraft's Shell (`shell`) hollows a body inward to a wall of the thickness, opening the faces
//! picked by points on them.
use crate::solids::*;

pub fn build() -> R<Body> {
    let outline = [
        (0.0, 0.0), (10.0, 0.0), (10.0, 4.0), (14.0, 4.0), (14.0, 0.0), (24.0, 0.0),
        (24.0, 10.0), (14.0, 10.0), (14.0, 6.0), (10.0, 6.0), (10.0, 10.0), (0.0, 10.0),
    ];
    let plate = extrude(&Plane::XY, polygon(&outline), 0.0, 6.0)?;
    solidcraft_kernel::shell(&plate, &[v3(5.0, 5.0, 6.0)], 1.5)
}
