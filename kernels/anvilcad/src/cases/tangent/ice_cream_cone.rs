//! A ball of radius 15 about the origin united with a cone on the z axis from its apex at
//! (0, 0, −25) to its base of radius 12 at z = −9, tangent to the sphere along the base's rim.
use anvil_brep::Solid;

use crate::shared::{at, ball, cone, union};

pub fn build() -> Result<Solid, String> {
    union(&ball([0., 0., 0.], 15.)?, &cone(&at([0., 0., -25.]), 0., 12., 16.)?)
}
