//! The box from the origin to (40, 30, 8) drilled through along z by holes of radius 3 at
//! (15, 15) and (25, 15), both walls offset 3 into the plate, until the holes merge.
use anvil_brep::Solid;

use crate::shared::{at, block, cut_all, cylinder, move_faces};

pub fn build() -> Result<Solid, String> {
    let holes = [cylinder(&at([15., 15., 0.]), 3., -1., 9.), cylinder(&at([25., 15., 0.]), 3., -1., 9.)];
    let drilled = cut_all(block([0., 0., 0.], [40., 30., 8.])?, holes)?;
    move_faces(&drilled, &[([12., 15., 4.], -3.), ([28., 15., 4.], -3.)])
}
