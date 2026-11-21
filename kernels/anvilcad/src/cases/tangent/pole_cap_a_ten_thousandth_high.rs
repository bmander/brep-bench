//! The cap of the ball of radius 5 about the origin above z = 4.9999, 10⁻⁴ high.
use anvil_brep::Solid;

use crate::shared::{ball, beyond, subtract};

pub fn build() -> Result<Solid, String> {
    // everything below the plane, past the ball on every side
    subtract(&ball([0., 0., 0.], 5.)?, &beyond([0., 0., 4.9999], [0., 0., -1.], 10.)?)
}
