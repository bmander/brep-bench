use crate::solids::*;

pub fn build() -> R<Body> {
    solidcraft_kernel::sphere(Vec3::ZERO, 5.0)
}
