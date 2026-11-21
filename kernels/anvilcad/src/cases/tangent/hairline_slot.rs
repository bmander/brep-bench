//! A cube of side 10 from the origin with a slot a millionth wide sawn into its top: the block
//! from x = 5 to 5.000001, through it in y, from z = 5 up.
use anvil_brep::Solid;

use crate::shared::{block, subtract};

pub fn build() -> Result<Solid, String> {
    // the saw runs past the cube's front, back and top
    subtract(&block([0., 0., 0.], [10., 10., 10.])?, &block([5., -1., 5.], [5.000001, 11., 11.])?)
}
