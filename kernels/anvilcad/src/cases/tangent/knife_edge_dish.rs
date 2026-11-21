//! A cylinder of radius 5 on the z axis from z = 0 to 10, less the ball of radius 5 about the
//! middle of its top, (0, 0, 10).
use anvil_brep::{Frame, Solid};

use crate::shared::{ball, cylinder, subtract};

pub fn build() -> Result<Solid, String> {
    subtract(&cylinder(&Frame::world(), 5., 0., 10.)?, &ball([0., 0., 10.], 5.)?)
}
