//! A cube of side 10 from the origin and the ball of radius 5 inscribed in it, about (5, 5, 5):
//! their common part.
use anvil_brep::Solid;

use crate::shared::{ball, block, intersect};

pub fn build() -> Result<Solid, String> {
    intersect(&block([0., 0., 0.], [10., 10., 10.])?, &ball([5., 5., 5.], 5.)?)
}
