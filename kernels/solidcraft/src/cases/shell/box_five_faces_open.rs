//! SolidCraft's Shell (`shell`) hollows a body inward to a wall of the thickness, opening the faces
//! picked by points on them.
use crate::solids::*;

pub fn build() -> R<Body> {
    let b = block((0.0, 0.0, 0.0), (10.0, 8.0, 6.0))?;
    solidcraft_kernel::shell(&b, &[v3(5.0, 4.0, 6.0), v3(0.0, 4.0, 3.0), v3(10.0, 4.0, 3.0), v3(5.0, 0.0, 3.0), v3(5.0, 8.0, 3.0)], 1.0)
}
