use crate::solids::*;

/// The L's section in x and z, extruded along +y from 0 to 10.
pub fn l_block() -> R<Body> {
    let l = polygon(&[(0.0, 0.0), (10.0, 0.0), (10.0, 3.0), (3.0, 3.0), (3.0, 10.0), (0.0, 10.0)]);
    // the plane of `xz` has its normal along −y: extrude toward −(−y)
    extrude(&xz(), l, -10.0, 0.0)
}

pub fn build() -> R<Body> {
    solidcraft_kernel::fillet(&l_block()?, &[v3(3.0, 5.0, 3.0)], 7.0)
}
