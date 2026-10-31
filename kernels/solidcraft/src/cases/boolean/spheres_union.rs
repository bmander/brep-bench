use crate::solids::*;

pub fn build() -> R<Body> {
    let a = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    let b = solidcraft_kernel::sphere(v3(6.0, 0.0, 0.0), 5.0)?;
    union(&a, &b)
}
