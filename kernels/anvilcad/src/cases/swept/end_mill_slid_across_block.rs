//! A block from (0, −5, 0) to (20, 5, 10) less what an upright end mill (radius 2, from z = 7 to
//! 15) passes through, slid along x from x = −5 to 25.
use anvil_brep::Solid;

use crate::shared::{at, block, cylinder, subtract, union};

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no sweep of a solid. A cylinder slid square to its axis fills itself at each
    // end and the box its shadow along the slide sweeps between
    let mill = |x: f64| cylinder(&at([x, 0., 0.]), 2., 7., 15.);
    let swept = union(&union(&mill(-5.)?, &block([-5., -2., 7.], [25., 2., 15.])?)?, &mill(25.)?)?;
    subtract(&block([0., -5., 0.], [20., 5., 10.])?, &swept)
}
