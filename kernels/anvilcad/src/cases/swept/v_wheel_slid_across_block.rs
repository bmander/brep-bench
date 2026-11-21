//! A block from (0, −5, 0) to (20, 5, 10) less what a sharp V wheel (the triangle (0, 8),
//! (∓3 tan 30°, 11) in (y, z) turned about the line through (−8, 0, 14) along y) passes through,
//! slid 36 along x.
use anvil_brep::{Solid, extrude};

use crate::shared::{block, e, frame, polygon, region, subtract, turned, union};

pub fn build() -> Result<Solid, String> {
    let t = 3. * 30f64.to_radians().tan();
    // the wheel: its section, as (distance from its axis, along its axis), turned about the axis
    let wheel = |x: f64| turned(&frame([x, 0., 14.], [0., 1., 0.], [0., 0., -1.])?, polygon(&[(6., 0.), (3., -t), (3., t)]));
    // anvil-brep has no sweep of a solid. Slid in its own plane the wheel fills itself at each end
    // and the prism of its shadow along the slide between: at each y the wheel reaches 6 − √3|y|
    // above and below its axis, so the shadow is the hexagon through (0, 8), (±√3, 11), (±√3, 17)
    // and (0, 20), in (y, z)
    let side = frame([-8., 0., 0.], [1., 0., 0.], [0., 1., 0.])?;
    let s3 = 3f64.sqrt();
    let shadow = region(&[polygon(&[(0., 8.), (s3, 11.), (s3, 17.), (0., 20.), (-s3, 17.), (-s3, 11.)])])?;
    let between = extrude::extrude(&side, &shadow, 0., 36.).map_err(e)?;
    let swept = union(&union(&wheel(-8.)?, &between)?, &wheel(28.)?)?;
    subtract(&block([0., -5., 0.], [20., 5., 10.])?, &swept)
}
