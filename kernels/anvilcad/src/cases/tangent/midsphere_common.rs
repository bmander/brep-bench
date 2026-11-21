//! The common part of the cube from (−5, −5, −5) to (5, 5, 5) and the ball of radius 5√2 about the
//! origin, which touches each of the cube's edges at its middle.
use anvil_brep::Solid;

use crate::shared::{ball, block, intersect};

pub fn build() -> Result<Solid, String> {
    intersect(&block([-5., -5., -5.], [5., 5., 5.])?, &ball([0., 0., 0.], 5. * 2f64.sqrt())?)
}
