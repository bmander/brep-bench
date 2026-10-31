use crate::solids::*;

pub fn build() -> R<Body> {
    let plate = block((0.0, -5.0, 0.0), (80.0, 5.0, 40.0))?;
    let pin = solidcraft_kernel::cylinder(v3(60.0, -6.0, 20.0), Vec3::Y, 8.0, 12.0)?;
    cut(&plate, &pin)
}
