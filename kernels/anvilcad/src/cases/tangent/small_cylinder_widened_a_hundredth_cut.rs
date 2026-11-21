//! A cylinder of radius 0.0007575 and height 0.0015 on the z axis from z = 0, less the cylinder of
//! radius 0.00075 between the same planes: a tube with a wall 0.0000075 thick.
use anvil_brep::{Frame, Solid};

use crate::shared::{cylinder, subtract};

pub fn build() -> Result<Solid, String> {
    let w = Frame::world();
    subtract(&cylinder(&w, 0.0007575, 0., 0.0015)?, &cylinder(&w, 0.00075, 0., 0.0015)?)
}
