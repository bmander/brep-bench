//! The lens common to two balls of radius 5 about the origin and (6, 0, 0).
use anvil_brep::Solid;

use crate::shared::{ball, intersect};

pub fn build() -> Result<Solid, String> {
    intersect(&ball([0., 0., 0.], 5.)?, &ball([6., 0., 0.], 5.)?)
}
