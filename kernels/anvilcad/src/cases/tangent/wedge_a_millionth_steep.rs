//! The cube from (−5, −5, −10) to (5, 5, 0) cut by the plane z = 10⁻⁶ y, keeping what lies above
//! it: a wedge 5·10⁻⁶ thick at most.
use anvil_brep::Solid;

use crate::shared::{beyond, block, subtract};

pub fn build() -> Result<Solid, String> {
    // everything below the plane, its normal (0, 10⁻⁶, −1) pointing away from the kept side
    subtract(&block([-5., -5., -10.], [5., 5., 0.])?, &beyond([0., 0., 0.], [0., 1e-6, -1.], 20.)?)
}
