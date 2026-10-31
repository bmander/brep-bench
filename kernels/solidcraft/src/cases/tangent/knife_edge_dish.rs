use crate::solids::*;

pub fn build() -> R<Body> {
    let drum = zcyl(0.0, 0.0, 5.0, 0.0, 10.0)?;
    cut(&drum, &solidcraft_kernel::sphere(v3(0.0, 0.0, 10.0), 5.0)?)
}
