//! The ball of radius 5 about the origin cut by the planes z = ±3, shrunk by 1: every face moved
//! 1 inward.
use anvil_brep::Solid;

use crate::shared::{ball, block, intersect, offset};

pub fn build() -> Result<Solid, String> {
    let barrel = intersect(&ball([0., 0., 0.], 5.)?, &block([-6., -6., -3.], [6., 6., 3.])?)?;
    offset(&barrel, -1.)
}
