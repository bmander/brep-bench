//! A block from the origin to (10, 4, 6), the two top edges along x rounded to radius 2, half the
//! top's width: the top becomes a half cylinder.
use anvil_brep::Solid;

use crate::shared::{block, fillet};

pub fn build() -> Result<Solid, String> {
    fillet(&block([0., 0., 0.], [10., 4., 6.])?, &[[5., 0., 6.], [5., 4., 6.]], 2.)
}
