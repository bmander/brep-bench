//! A cylinder of radius 5 on the z axis from z = 0 to 10, its top rim rounded to radius 1.
use anvil_brep::{Frame, Solid};

use crate::shared::{cylinder, fillet, ring};

pub fn build() -> Result<Solid, String> {
    fillet(&cylinder(&Frame::world(), 5., 0., 10.)?, &[ring(5., 10.)], 1.)
}
