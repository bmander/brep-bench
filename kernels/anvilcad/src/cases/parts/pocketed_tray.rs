//! A tray on its back: the block from (−36, 0, −24) to (36, 16, 24) less the pocket from
//! (−33, 0, −21) to (33, 13, 21), with four tube standoffs (radius 5, holes of radius 2) along y
//! from y = 5 to 13 at (±26, ±14), their holes blind at the floor.
use anvil_brep::Solid;

use crate::shared::{block, rod, subtract, union};

pub fn build() -> Result<Solid, String> {
    let mut tray = subtract(&block([-36., 0., -24.], [36., 16., 24.])?, &block([-33., -1., -21.], [33., 13., 21.])?)?;
    for (x, z) in [(26., 14.), (-26., 14.), (-26., -14.), (26., -14.)] {
        tray = union(&tray, &rod([x, 5., z], [x, 13., z], 5.)?)?;
        // the hole from the standoff's end down to the floor, and no further
        tray = subtract(&tray, &rod([x, 4., z], [x, 13., z], 2.)?)?;
    }
    Ok(tray)
}
