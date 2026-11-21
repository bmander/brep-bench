//! The union of two balls of radius 5 about the origin and (6, 0, 0).
use anvil_brep::Solid;

use crate::shared::{ball, union};

pub fn build() -> Result<Solid, String> {
    union(&ball([0., 0., 0.], 5.)?, &ball([6., 0., 0.], 5.)?)
}
