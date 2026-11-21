//! A cube of side 10 from the origin, drilled through along z by a hole of radius 3 at (5, 5).
use anvil_brep::Solid;

use crate::shared::{at, block, cylinder, subtract};

pub fn build() -> Result<Solid, String> {
    let cube = block([0., 0., 0.], [10., 10., 10.])?;
    // the drill runs past both faces, from z = −1 to z = 11
    let drill = cylinder(&at([5., 5., 0.]), 3., -1., 11.)?;
    subtract(&cube, &drill)
}
