//! Five balls each inside the one before: radius 10 about the origin less radius 8 about
//! (1, 0, 0), joined with radius 6 about (1, 1, 0) less radius 4 about (1, 1, 1), joined with
//! radius 2 about (1, 1.5, 1.5). Three bodies, each in the hollow of the one around it.
use anvil_brep::Solid;

use crate::shared::{ball, subtract, unite_into};

pub fn build() -> Result<Vec<Solid>, String> {
    let outer = subtract(&ball([0., 0., 0.], 10.)?, &ball([1., 0., 0.], 8.)?)?;
    let middle = subtract(&ball([1., 1., 0.], 6.)?, &ball([1., 1., 1.], 4.)?)?;
    let two = unite_into(vec![outer], middle)?;
    unite_into(two, ball([1., 1.5, 1.5], 2.)?)
}
