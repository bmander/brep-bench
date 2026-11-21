//! A cylinder of radius 5 on the z axis from z = 0 to 10, less one of radius 3 on the same axis
//! running exactly from z = 0 to 10: its ends lie in the first's.
use anvil_brep::{Frame, Solid};

use crate::shared::{cylinder, subtract};

pub fn build() -> Result<Solid, String> {
    let w = Frame::world();
    subtract(&cylinder(&w, 5., 0., 10.)?, &cylinder(&w, 3., 0., 10.)?)
}
