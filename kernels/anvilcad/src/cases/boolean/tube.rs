//! A cylinder of radius 5 on the z axis from z = 0 to 10, bored along the axis by a hole of
//! radius 3.
use anvil_brep::{Frame, Solid};

use crate::shared::{cylinder, subtract};

pub fn build() -> Result<Solid, String> {
    let w = Frame::world();
    // the drill runs past both ends, from z = −1 to z = 11
    subtract(&cylinder(&w, 5., 0., 10.)?, &cylinder(&w, 3., -1., 11.)?)
}
