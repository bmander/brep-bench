//! A ball of radius 5 about the origin, everything below the xy plane cut away.
use anvil_brep::Solid;

use crate::shared::{ball, block, subtract};

pub fn build() -> Result<Solid, String> {
    subtract(&ball([0., 0., 0.], 5.)?, &block([-6., -6., -6.], [6., 6., 0.])?)
}
