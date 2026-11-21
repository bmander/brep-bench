//! A disk of radius 20 from z = −5 to z = 0, with six holes of radius 2 through it along z, on
//! the circle of radius 12, 60° apart from (12, 0); each hole its own cut.
use anvil_brep::{Frame, Solid};

use crate::shared::{at, cut_all, cylinder, turn_z};

pub fn build() -> Result<Solid, String> {
    let disk = cylinder(&Frame::world(), 20., -5., 0.)?;
    // the first hole, running past both faces, and the others it turned about z
    let first = cylinder(&at([12., 0., 0.]), 2., -6., 1.)?;
    cut_all(disk, (0..6).map(|k| Ok(first.transformed(&turn_z(60. * k as f64)))))
}
