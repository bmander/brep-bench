//! SolidCraft's Shell (`shell`) hollows a body inward to a wall of the thickness, opening the faces
//! picked by points on them.
use crate::solids::*;

pub fn build() -> R<Body> {
    let b = block((0.0, 0.0, 0.0), (20.0, 20.0, 10.0))?;
    // the bore's cylinder runs past both faces
    let drilled = cut(&b, &zcyl(10.0, 10.0, 3.0, -1.0, 11.0)?)?;
    solidcraft_kernel::shell(&drilled, &[v3(1.0, 1.0, 10.0)], 1.0)
}
