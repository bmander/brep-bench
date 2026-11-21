//! A cylinder of radius 5 on the z axis from z = 0 to 10, less the cylinder of radius 4.9999 between
//! the same planes: a tube whose wall is 0.0001 thick.
use anvil_brep::{Frame, Solid};

use crate::shared::{cylinder, subtract};

pub fn build() -> Result<Solid, String> {
    let w = Frame::world();
    subtract(&cylinder(&w, 5., 0., 10.)?, &cylinder(&w, 4.9999, 0., 10.)?)
}
