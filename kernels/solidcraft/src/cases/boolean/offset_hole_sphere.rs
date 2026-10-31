use crate::solids::*;

pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 20.0)?;
    let hole = solidcraft_kernel::cylinder(v3(5.0, -25.0, 0.0), Vec3::Y, 6.0, 50.0)?;
    cut(&ball, &hole)
}
