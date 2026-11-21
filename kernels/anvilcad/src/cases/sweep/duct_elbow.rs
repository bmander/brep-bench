//! A hollow square duct, 18 × 18 with a 14 × 14 hole, swept a quarter turn along the arc of radius
//! 30 about the z axis from (30, 0, 0) to (0, 30, 0).
use std::f64::consts::FRAC_PI_2;

use anvil_brep::Solid;
use anvil_brep::sweep::{PathSegment, sweep};

use crate::shared::{e, frame, one, rect, region, v3};

pub fn build() -> Result<Solid, String> {
    // the section square to the path at its start: in the xz plane at x = 30, its sketch x along
    // world x and its sketch y along world z
    let start = frame([30., 0., 0.], [0., 1., 0.], [1., 0., 0.])?;
    // the frame's y is z × x = (0, 1, 0) × (1, 0, 0) = −z, so the sketch's y runs down: the square
    // is symmetric, which does not matter here
    let section = region(&[rect((-9., -9.), (9., 9.)), rect((-7., -7.), (7., 7.))])?;
    let path = [PathSegment::Arc { center: v3(0., 0., 0.), axis: v3(0., 0., 1.), start: v3(30., 0., 0.), angle: FRAC_PI_2 }];
    one(sweep(&start, &section, &path).map_err(e)?)
}
