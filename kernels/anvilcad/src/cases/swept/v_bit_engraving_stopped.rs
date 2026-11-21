//! A block from (0, −10, −10) to (30, 10, 0) less what a 90° V bit (its point at (5, 0, −2),
//! opening up to radius 5 at z = 3) passes through, slid 15 along x.
use anvil_brep::{Solid, extrude};

use crate::shared::{at, block, cone, e, frame, polygon, region, subtract, union};

pub fn build() -> Result<Solid, String> {
    let bit = |x: f64| cone(&at([x, 0., -2.]), 0., 5., 5.);
    // anvil-brep has no sweep of a solid. Slid square to its axis the cone fills itself at each end
    // and the prism of its shadow along the slide between, the triangle |y| ≤ z + 2 up to z = 3
    let side = frame([5., 0., 0.], [1., 0., 0.], [0., 1., 0.])?;
    let shadow = region(&[polygon(&[(0., -2.), (5., 3.), (-5., 3.)])])?;
    let between = extrude::extrude(&side, &shadow, 0., 15.).map_err(e)?;
    let swept = union(&union(&bit(5.)?, &between)?, &bit(20.)?)?;
    subtract(&block([0., -10., -10.], [30., 10., 0.])?, &swept)
}
