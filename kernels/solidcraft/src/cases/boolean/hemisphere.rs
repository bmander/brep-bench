use crate::solids::*;

pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    cut(&ball, &block((-10.0, -10.0, -10.0), (10.0, 10.0, 0.0))?)
}
