//! A cube of side 10 from the origin, united with a cylinder of radius 5 and height 10 on the xy
//! plane whose axis is at x = 10, y = 5: it touches the faces y = 0 and y = 10 along the cube's
//! edges at x = 10.
use anvil_brep::Solid;

use crate::shared::{at, block, cylinder, union};

pub fn build() -> Result<Solid, String> {
    let cube = block([0., 0., 0.], [10., 10., 10.])?;
    let boss = cylinder(&at([10., 5., 0.]), 5., 0., 10.)?;
    union(&cube, &boss)
}
