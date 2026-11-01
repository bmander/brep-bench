//! SolidCraft's Shell (`shell`) hollows a body inward to a wall of the thickness, opening the faces
//! picked by points on them.
use crate::solids::*;

pub fn build() -> R<Body> {
    let torus = solidcraft_kernel::torus(Vec3::ZERO, 6.0, 2.0)?;
    // everything where y < 0 cut away
    let half = cut(&torus, &block((-10.0, -10.0, -5.0), (10.0, 0.0, 5.0))?)?;
    solidcraft_kernel::shell(&half, &[v3(6.0, 0.0, 0.0), v3(-6.0, 0.0, 0.0)], 0.5)
}
