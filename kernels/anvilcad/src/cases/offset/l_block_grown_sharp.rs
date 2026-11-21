//! The L through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15) extruded 8 up z, grown by 1
//! with its edges kept sharp: every face moved 1 outward, the moved faces extended until they meet.
use anvil_brep::Solid;

use crate::shared::{offset, prism};

pub fn build() -> Result<Solid, String> {
    let l = prism(&[(0., 0.), (20., 0.), (20., 5.), (5., 5.), (5., 15.), (0., 15.)], 0., 8.)?;
    offset(&l, 1.)
}
