//! A cube of side 10 from the origin, the edge where x = 10 meets y = 10 rounded to radius 2.
use anvil_brep::Solid;

use crate::shared::{block, fillet};

pub fn build() -> Result<Solid, String> {
    fillet(&block([0., 0., 0.], [10., 10., 10.])?, &[[10., 10., 5.]], 2.)
}
