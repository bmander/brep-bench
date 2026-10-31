use crate::solids::*;

pub fn build() -> R<Body> {
    let cube = block((0.0, 0.0, 0.0), (10.0, 10.0, 10.0))?;
    solidcraft_kernel::fillet(&cube, &[v3(10.0, 10.0, 5.0)], 2.0)
}
