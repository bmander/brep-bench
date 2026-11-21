//! A cube of side 0.0015 from the origin and a copy of it raised 0.000015 along z: their union.
use anvil_brep::Solid;

use crate::shared::{block, union};

pub fn build() -> Result<Solid, String> {
    let s = 0.0015;
    union(&block([0., 0., 0.], [s, s, s])?, &block([0., 0., 0.000015], [s, s, s + 0.000015])?)
}
