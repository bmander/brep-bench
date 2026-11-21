//! The upper half of a ball of radius 5 about the origin, its rim chamfered 1. in on the flat face,
//! at 45.° to it.
use anvil_brep::Solid;

use crate::shared::{ball, beyond, chamfer_angle, ring, subtract};

pub fn build() -> Result<Solid, String> {
    let half = subtract(&ball([0., 0., 0.], 5.)?, &beyond([0., 0., 0.], [0., 0., -1.], 10.)?)?;
    chamfer_angle(&half, ring(5., 0.), ring(1., 0.), 1., 45.)
}
