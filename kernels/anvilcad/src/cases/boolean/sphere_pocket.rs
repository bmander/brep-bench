//! A block from (−10, −10, −10) to (10, 10, 0) less a ball of radius 5 about the origin: a
//! hemispherical pocket in its top.
use anvil_brep::Solid;

use crate::shared::{ball, block, subtract};

pub fn build() -> Result<Solid, String> {
    subtract(&block([-10., -10., -10.], [10., 10., 0.])?, &ball([0., 0., 0.], 5.)?)
}
