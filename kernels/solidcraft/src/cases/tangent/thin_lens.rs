use crate::solids::*;

pub fn build() -> R<Body> {
    let a = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    let b = solidcraft_kernel::sphere(v3(9.999999, 0.0, 0.0), 5.0)?;
    common(&a, &b)
}
