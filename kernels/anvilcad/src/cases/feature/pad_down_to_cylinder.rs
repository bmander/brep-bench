//! A cylinder of radius 10 along the x axis from x = −20 to 20, with the square |x|, |y| ≤ 3 at
//! z = 15 extruded down until it meets the cylinder's side, joined to it.
use anvil_brep::Solid;

use crate::shared::{block, rod, union};

pub fn build() -> Result<Solid, String> {
    // the pad extruded down into the cylinder; joined, what lies inside it is the cylinder's
    union(&rod([-20., 0., 0.], [20., 0., 0.], 10.)?, &block([-3., -3., 5.], [3., 3., 15.])?)
}
