//! A cylinder of radius 0.00075 and height 0.0015 on the z axis from z = 0 and a copy of it raised
//! 0.000015 along z: their union.
use anvil_brep::{Frame, Solid};

use crate::shared::{cylinder, union};

pub fn build() -> Result<Solid, String> {
    let w = Frame::world();
    union(&cylinder(&w, 0.00075, 0., 0.0015)?, &cylinder(&w, 0.00075, 0.000015, 0.001515)?)
}
