//! A ball of radius 5 about the origin less the cap beyond the plane x + z = 5.
use anvil_brep::Solid;

use crate::shared::{ball, beyond, subtract};

pub fn build() -> Result<Solid, String> {
    // the ball revolved about z (anvil-brep's spheres have no poles to place)
    subtract(&ball([0., 0., 0.], 5.)?, &beyond([2.5, 0., 2.5], [1., 0., 1.], 10.)?)
}
