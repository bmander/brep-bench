//! The union of two balls of radius 5 about the origin and (4.8, 0, 3.6).
use anvil_brep::Solid;

use crate::shared::{ball, union};

pub fn build() -> Result<Solid, String> {
    // each revolved about the vertical through its center
    union(&ball([0., 0., 0.], 5.)?, &ball([4.8, 0., 3.6], 5.)?)
}
