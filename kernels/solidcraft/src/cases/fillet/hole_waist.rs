use crate::solids::*;

pub fn build() -> R<Body> {
    let plate = block((-10.0, -10.0, 0.0), (10.0, 10.0, 2.0))?;
    let holed = cut(&plate, &zcyl(0.0, 0.0, 3.0, -1.0, 3.0)?)?;
    solidcraft_kernel::fillet(&holed, &[v3(0.0, 3.0, 2.0), v3(0.0, 3.0, 0.0)], 1.0)
}
