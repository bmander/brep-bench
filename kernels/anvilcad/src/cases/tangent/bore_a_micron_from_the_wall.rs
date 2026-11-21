//! A cylinder of radius 5 on the z axis from z = 0 to 10, bored through along z by a hole of
//! radius 4 whose axis is x = 0.999999, y = 0: the wall between them 10⁻⁶ thick.
use anvil_brep::{Frame, Solid};

use crate::shared::{at, cylinder, subtract};

pub fn build() -> Result<Solid, String> {
    // the drill runs past both ends, from z = −1 to z = 11
    subtract(&cylinder(&Frame::world(), 5., 0., 10.)?, &cylinder(&at([0.999999, 0., 0.]), 4., -1., 11.)?)
}
