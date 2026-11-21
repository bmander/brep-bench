//! A ball of radius 50000 about the origin united with one of radius 40000 about
//! (20000, 30000, 60000).
use anvil_brep::Solid;

use crate::shared::{ball, union};

pub fn build() -> Result<Solid, String> {
    union(&ball([0., 0., 0.], 50000.)?, &ball([20000., 30000., 60000.], 40000.)?)
}
