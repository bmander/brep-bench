use crate::solids::*;

pub fn build() -> R<Body> {
    let slab = block((-10.0, -10.0, -10.0), (10.0, 10.0, 0.0))?;
    cut(&slab, &solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?)
}
