use crate::solids::*;

pub fn build() -> R<Body> {
    let cube = block((0.0, 0.0, 0.0), (10.0, 10.0, 10.0))?;
    union(&cube, &solidcraft_kernel::sphere(v3(5.0, 5.0, 5.0), 5.0)?)
}
