//! SolidCraft's Shell (`shell`) hollows a body inward to a wall of the thickness, opening the faces
//! picked by points on them.
use crate::solids::*;

pub fn build() -> R<Body> {
    let c = zcyl(0.0, 0.0, 5.0, 0.0, 10.0)?;
    solidcraft_kernel::shell(&c, &[v3(0.0, 0.0, 10.0)], 1.0)
}
