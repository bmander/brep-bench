//! A circle of radius 2 swept a quarter turn along the arc of radius 10 about the z axis from
//! (10, 0, 0) to (0, 10, 0), square to the path, its ends flat.
use std::f64::consts::FRAC_PI_2;

use anvil_brep::Solid;
use anvil_brep::sweep::{PathSegment, sweep};

use crate::shared::{circle, e, frame, one, region, v3};

pub fn build() -> Result<Solid, String> {
    // the section sits at the path's start, square to it: its plane's normal is the path's +y
    let start = frame([10., 0., 0.], [0., 1., 0.], [1., 0., 0.])?;
    let path = [PathSegment::Arc { center: v3(0., 0., 0.), axis: v3(0., 0., 1.), start: v3(10., 0., 0.), angle: FRAC_PI_2 }];
    one(sweep(&start, &region(&[circle((0., 0.), 2.)])?, &path).map_err(e)?)
}
