//! A cylinder of radius 0.00075 and height 0.0015 on the z axis from z = 0 and a copy of it slid
//! 0.000015 along x: their common part.
use anvil_brep::{Frame, Solid};

use crate::shared::{at, cylinder, intersect};

pub fn build() -> Result<Solid, String> {
    intersect(&cylinder(&Frame::world(), 0.00075, 0., 0.0015)?, &cylinder(&at([0.000015, 0., 0.]), 0.00075, 0., 0.0015)?)
}
