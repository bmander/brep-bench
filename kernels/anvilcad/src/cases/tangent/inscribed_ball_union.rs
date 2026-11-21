//! A cube of side 10 from the origin and the ball of radius 5 inscribed in it, about (5, 5, 5):
//! their union.
use anvil_brep::Solid;

use crate::shared::{ball, block, union};

pub fn build() -> Result<Solid, String> {
    union(&block([0., 0., 0.], [10., 10., 10.])?, &ball([5., 5., 5.], 5.)?)
}
