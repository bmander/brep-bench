//! A plate from the origin to (20, 10, 2), the two edges of its face y = 10 along x rounded to
//! radius 0.9999, leaving a strip 0.0002 tall between them.
use anvil_brep::Solid;

use crate::shared::{block, fillet};

pub fn build() -> Result<Solid, String> {
    fillet(&block([0., 0., 0.], [20., 10., 2.])?, &[[10., 10., 0.], [10., 10., 2.]], 0.9999)
}
