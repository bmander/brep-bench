//! The box from the origin to (40, 30, 10) drilled through along z by a hole of radius 5 at
//! (20, 15), its top moved up 6.
use anvil_brep::Solid;

use crate::shared::{at, block, cylinder, move_faces, subtract};

pub fn build() -> Result<Solid, String> {
    let drilled = subtract(&block([0., 0., 0.], [40., 30., 10.])?, &cylinder(&at([20., 15., 0.]), 5., -1., 11.)?)?;
    move_faces(&drilled, &[([5., 5., 10.], 6.)])
}
