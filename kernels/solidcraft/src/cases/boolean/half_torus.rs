use crate::solids::*;

pub fn build() -> R<Body> {
    let ring = solidcraft_kernel::torus(Vec3::ZERO, 10.0, 2.0)?;
    cut(&ring, &block((-20.0, -20.0, -5.0), (0.0, 20.0, 5.0))?)
}
