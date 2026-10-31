use crate::solids::*;

pub fn build() -> R<Body> {
    let cube = block((0.0, 0.0, 0.0), (10.0, 10.0, 10.0))?;
    // radius 1 at the end nearest (10, 10, 0), growing linearly to 2 at the other
    solidcraft_kernel::fillet_variable(&cube, v3(10.0, 10.0, 5.0), v3(10.0, 10.0, 0.0), 1.0, 2.0)
}
