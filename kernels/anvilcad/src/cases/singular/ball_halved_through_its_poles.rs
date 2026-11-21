//! A ball of radius 5 about the origin, everything on the side x < 0 cut away.
use anvil_brep::Solid;

use crate::shared::{ball, beyond, subtract};

pub fn build() -> Result<Solid, String> {
    subtract(&ball([0., 0., 0.], 5.)?, &beyond([0., 0., 0.], [-1., 0., 0.], 10.)?)
}
