//! A ball of radius 0.005 about the origin united with one of radius 0.004 about
//! (0.002, 0.003, 0.006).
use anvil_brep::Solid;

use crate::shared::{ball, union};

pub fn build() -> Result<Solid, String> {
    union(&ball([0., 0., 0.], 0.005)?, &ball([0.002, 0.003, 0.006], 0.004)?)
}
