use crate::solids::*;
use solidcraft_kernel::ChamferSide;

pub fn build() -> R<Body> {
    let b = block((0.0, 0.0, 0.0), (20.0, 10.0, 6.0))?;
    let rim = [v3(10.0, 0.0, 6.0), v3(10.0, 10.0, 6.0), v3(0.0, 5.0, 6.0), v3(20.0, 5.0, 6.0)];
    // 1 across the first face (the top: the one facing up most), 2 down the other
    solidcraft_kernel::chamfer_sides(&b, &rim, 1.0, ChamferSide::Distance(2.0), false)
}
