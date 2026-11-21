//! A C-shaped bracket, the section (0, 0), (40, 0), (40, 6), (6, 6), (6, 24), (40, 24), (40, 30),
//! (0, 30) in xz from y = −10 to 10, with a hole of radius 4 on the vertical through (28, 0)
//! drilled down through its upper arm and no further.
use anvil_brep::Solid;

use crate::shared::{at, cylinder, prism_xz, subtract};

pub fn build() -> Result<Solid, String> {
    let c = prism_xz(&[(0., 0.), (40., 0.), (40., 6.), (6., 6.), (6., 24.), (40., 24.), (40., 30.), (0., 30.)], -10., 10.)?;
    // the drill from above the arm down to the gap under it, z = 24, the first wall it meets
    subtract(&c, &cylinder(&at([28., 0., 0.]), 4., 20., 31.)?)
}
