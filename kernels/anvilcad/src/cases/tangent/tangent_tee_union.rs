//! A cylinder of radius 5 along the x axis from x = −15 to 15, and one of radius 3 parallel to y
//! on the line x = 0, z = 2, from y = −15 to 15, touching it from inside at (0, 0, 5): their
//! union.
use anvil_brep::Solid;

use crate::shared::{rod, union};

pub fn build() -> Result<Solid, String> {
    union(&rod([-15., 0., 0.], [15., 0., 0.], 5.)?, &rod([0., -15., 2.], [0., 15., 2.], 3.)?)
}
