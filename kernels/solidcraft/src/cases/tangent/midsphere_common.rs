use crate::solids::*;

pub fn build() -> R<Body> {
    let cube = block((-5.0, -5.0, -5.0), (5.0, 5.0, 5.0))?;
    common(&cube, &solidcraft_kernel::sphere(Vec3::ZERO, 5.0 * 2f64.sqrt())?)
}
