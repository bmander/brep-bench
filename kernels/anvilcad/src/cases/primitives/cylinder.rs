//! A cylinder of radius 5 about the z axis, from z = 0 to z = 10.
use anvil_brep::{Frame, Solid};

use crate::shared::cylinder;

pub fn build() -> Result<Solid, String> {
    // a circle of radius 5 in the xy plane, extruded from z = 0 to z = 10
    cylinder(&Frame::world(), 5., 0., 10.)
}
