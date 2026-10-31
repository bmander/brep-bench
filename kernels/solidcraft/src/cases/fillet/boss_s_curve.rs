use crate::solids::*;

pub fn build() -> R<Body> {
    let plate = block((-20.0, -20.0, 0.0), (20.0, 20.0, 4.0))?;
    let boss = union(&plate, &zcyl(0.0, 0.0, 5.0, 4.0, 8.0)?)?;
    // the rim at z = 8 and the root at z = 4, each picked by a point on it
    solidcraft_kernel::fillet(&boss, &[v3(0.0, 5.0, 8.0), v3(0.0, 5.0, 4.0)], 2.0)
}
