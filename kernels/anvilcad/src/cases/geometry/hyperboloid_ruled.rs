//! The solid between the circles of radius 5 about the z axis at z = −5 and z = 5, its side ruled
//! by lines from each point of the lower circle to the point a quarter turn on round the upper.
use anvil_brep::Solid;
use anvil_brep::loft::{Section, loft_ruled};

use crate::shared::{at, circle, e, v3};

pub fn build() -> Result<Solid, String> {
    let lower = Section::Loop { frame: at([0., 0., -5.]), outline: circle((0., 0.), 5.) };
    let upper = Section::Loop { frame: at([0., 0., 5.]), outline: circle((0., 0.), 5.) };
    // each circle starts at its pick: (5, 0) below joins (0, 5) above, a quarter turn on
    loft_ruled(&lower, &upper, [Some(v3(5., 0., -5.)), Some(v3(0., 5., 5.))]).map_err(e)
}
