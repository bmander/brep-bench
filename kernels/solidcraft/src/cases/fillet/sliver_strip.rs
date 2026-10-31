use crate::solids::*;

pub fn build() -> R<Body> {
    let plate = block((0.0, 0.0, 0.0), (20.0, 10.0, 2.0))?;
    solidcraft_kernel::fillet(&plate, &[v3(10.0, 10.0, 0.0), v3(10.0, 10.0, 2.0)], 0.9999)
}
