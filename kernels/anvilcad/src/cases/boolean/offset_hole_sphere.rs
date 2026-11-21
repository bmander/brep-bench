//! A ball of radius 20 about the origin, bored through parallel to y by a hole of radius 6 whose
//! axis is the line x = 5, z = 0.
use anvil_brep::Solid;

use crate::shared::{ball, rod, subtract};

pub fn build() -> Result<Solid, String> {
    // the drill runs past the ball at both ends
    subtract(&ball([0., 0., 0.], 20.)?, &rod([5., -21., 0.], [5., 21., 0.], 6.)?)
}
