use crate::solids::*;

pub fn build() -> R<Body> {
    let a = solidcraft_kernel::cylinder(v3(-15.0, 0.0, 0.0), Vec3::X, 5.0, 30.0)?;
    let b = solidcraft_kernel::cylinder(v3(0.0, -15.0, 0.0), Vec3::Y, 5.0, 30.0)?;
    common(&a, &b)
}
