use crate::solids::*;

pub fn build() -> R<Body> {
    solidcraft_kernel::torus(Vec3::ZERO, 10.0, 2.0)
}
