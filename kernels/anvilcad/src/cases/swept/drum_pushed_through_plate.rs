//! A plate from (0, −5, 0) to (20, 5, 4) less what a drum on its side (radius 2, its axis
//! y = 0, z = −5 from x = 6 to 14) passes through, pushed 16 straight up.
use anvil_brep::Solid;

use crate::shared::{block, rod, subtract, union};

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no sweep of a solid. Pushed square to its axis the drum fills itself at each
    // end and the box its shadow along the push sweeps between
    let drum = |z: f64| rod([6., 0., z], [14., 0., z], 2.);
    let swept = union(&union(&drum(-5.)?, &block([6., -2., -5.], [14., 2., 11.])?)?, &drum(11.)?)?;
    subtract(&block([0., -5., 0.], [20., 5., 4.])?, &swept)
}
