//! A ball of radius 5 about the origin.
use anvil_brep::Solid;

use crate::shared::ball;

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no sphere primitive: a half disc of radius 5 revolved about its diameter
    ball([0., 0., 0.], 5.)
}
