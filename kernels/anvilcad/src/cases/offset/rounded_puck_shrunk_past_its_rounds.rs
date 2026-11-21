//! A cylinder of radius 6 on the z axis from z = 0 to 10, both rims rounded to radius 2, shrunk by
//! 3..
use anvil_brep::{Frame, Solid};

use crate::shared::{cylinder, fillet, offset, ring};

pub fn build() -> Result<Solid, String> {
    let p = fillet(&cylinder(&Frame::world(), 6., 0., 10.)?, &[ring(6., 0.), ring(6., 10.)], 2.)?;
    offset(&p, -3.)
}
