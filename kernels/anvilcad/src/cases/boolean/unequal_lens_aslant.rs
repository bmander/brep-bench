//! The lens common to a ball of radius 5.511 about the origin and one of radius 3.317 about
//! (4.231, 3.467, 2.653).
use anvil_brep::Solid;

use crate::shared::{ball, intersect};

pub fn build() -> Result<Solid, String> {
    intersect(&ball([0., 0., 0.], 5.511)?, &ball([4.231, 3.467, 2.653], 3.317)?)
}
