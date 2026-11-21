//! An L-block, the foot from the origin to (20, 10, 6) and the upright from the origin to
//! (6, 10, 20): its inside edge rounded to radius 2, then the chain the end face y = 0 is left
//! with, up the upright, round that round and along the foot, rounded to radius 2.
use std::f64::consts::FRAC_1_SQRT_2;

use anvil_brep::Solid;

use crate::shared::{block, fillet, union};

pub fn build() -> Result<Solid, String> {
    let l = union(&block([0., 0., 0.], [20., 10., 6.])?, &block([0., 0., 0.], [6., 10., 20.])?)?;
    let inside = fillet(&l, &[[6., 5., 6.]], 2.)?;
    // the chain: up the upright, the quarter circle about (8, 0, 8), along the foot
    let q = 8. - 2. * FRAC_1_SQRT_2;
    fillet(&inside, &[[6., 0., 14.], [q, 0., q], [14., 0., 6.]], 2.)
}
