//! The box from the origin to (40, 30, 8) drilled through along z by a hole of radius 5 at
//! (20, 15), the hole's wall offset 2 into the plate.
use anvil_brep::Solid;

use crate::shared::{at, block, cylinder, move_faces, subtract};

pub fn build() -> Result<Solid, String> {
    let drilled = subtract(&block([0., 0., 0.], [40., 30., 8.])?, &cylinder(&at([20., 15., 0.]), 5., -1., 9.)?)?;
    // the wall faces the axis, out of the plate: into the plate is 2 inward
    move_faces(&drilled, &[([25., 15., 4.], -2.)])
}
