//! The solid inside the hyperboloid of one sheet x² + y² ≤ 16(1 + z²/36), between z = −6 and 6.
use anvil_brep::Solid;
use anvil_brep::loft::{Section, loft_ruled};

use crate::shared::{at, circle, e, v3};

pub fn build() -> Result<Solid, String> {
    // anvil-brep cannot revolve a hyperbola, but a hyperboloid of one sheet is ruled: the lines
    // from each point of its end circle (radius 4√2) to the point a quarter turn on round the other
    // pass the axis at the waist's radius 4√2·cos 45° = 4, and sweep exactly that surface
    let r = 4. * 2f64.sqrt();
    let lower = Section::Loop { frame: at([0., 0., -6.]), outline: circle((0., 0.), r) };
    let upper = Section::Loop { frame: at([0., 0., 6.]), outline: circle((0., 0.), r) };
    loft_ruled(&lower, &upper, [Some(v3(r, 0., -6.)), Some(v3(0., r, 6.))]).map_err(e)
}
