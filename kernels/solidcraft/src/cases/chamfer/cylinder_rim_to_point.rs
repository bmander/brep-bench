use crate::solids::*;

pub fn build() -> R<Body> {
    let c = zcyl(0.0, 0.0, 5.0, 0.0, 10.0)?;
    solidcraft_kernel::chamfer(&c, &[v3(0.0, 5.0, 10.0)], 5.0)
}
