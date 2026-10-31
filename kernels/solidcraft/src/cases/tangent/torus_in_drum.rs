use crate::solids::*;

pub fn build() -> R<Body> {
    let ring = solidcraft_kernel::torus(Vec3::ZERO, 10.0, 3.0)?;
    union(&ring, &zcyl(0.0, 0.0, 13.0, -1.0, 1.0)?)
}
