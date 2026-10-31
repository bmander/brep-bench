use crate::solids::*;

pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    union(&ball, &zcyl(0.0, 0.0, 5.0, -3.0, 3.0)?)
}
