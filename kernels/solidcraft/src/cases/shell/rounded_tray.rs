//! SolidCraft's Shell (`shell`) hollows a body inward to a wall of the thickness, opening the faces
//! picked by points on them.
use crate::solids::*;

pub fn build() -> R<Body> {
    let b = block((0.0, 0.0, 0.0), (20.0, 14.0, 10.0))?;
    // the box's four vertical edges, each picked by its middle, rounded to radius 2
    let edges = [v3(0.0, 0.0, 5.0), v3(20.0, 0.0, 5.0), v3(20.0, 14.0, 5.0), v3(0.0, 14.0, 5.0)];
    let plate = solidcraft_kernel::fillet(&b, &edges, 2.0)?;
    solidcraft_kernel::shell(&plate, &[v3(10.0, 7.0, 10.0)], 1.0)
}
