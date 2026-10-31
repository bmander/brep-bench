use crate::solids::*;

pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    cut(&ball, &zcyl(0.0, 0.0, 3.0, -6.0, 6.0)?)
}
