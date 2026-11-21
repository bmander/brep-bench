//! A plate from (0, −5, 0) to (20, 5, 4) less what a drill (radius 2, a 118° point at (10, 0, 6),
//! up to z = 14) passes through, pushed 20 down along its axis.
use anvil_brep::Solid;

use crate::shared::{at, block, cone, cylinder, subtract, union};

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no sweep of a solid. Pushed along its own axis the drill fills its own
    // cylinder lengthened by the push, from where its point ends up to where its top starts
    let shoulder = 2. / 59f64.to_radians().tan();
    let point = cone(&at([10., 0., 6. - 20.]), 0., 2., shoulder)?;
    let swept = union(&point, &cylinder(&at([10., 0., 0.]), 2., 6. - 20. + shoulder, 14.)?)?;
    subtract(&block([0., -5., 0.], [20., 5., 4.])?, &swept)
}
