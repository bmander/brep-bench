use crate::solids::*;

pub fn build() -> R<Body> {
    solidcraft_kernel::cylinder(Vec3::ZERO, Vec3::Z, 5.0, 10.0)
}
