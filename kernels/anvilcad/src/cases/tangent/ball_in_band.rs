//! A ball of radius 5 about the origin united with a cylinder of radius 5 on the z axis from
//! z = −3 to 3, which it touches from inside along the equator.
use anvil_brep::{Frame, Solid};

use crate::shared::{ball, cylinder, union};

pub fn build() -> Result<Solid, String> {
    union(&ball([0., 0., 0.], 5.)?, &cylinder(&Frame::world(), 5., -3., 3.)?)
}
