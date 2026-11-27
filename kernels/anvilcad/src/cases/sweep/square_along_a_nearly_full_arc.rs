//! The square from (9, 0, −1) to (11, 0, 1) swept 350° along the arc of radius 10 about the z axis
//! from (10, 0, 0), square to the path, its ends flat.
use anvil_brep::Solid;
use anvil_brep::sweep::{PathSegment, sweep};

use crate::shared::{e, frame, one, rect, region, v3};

pub fn build() -> Result<Solid, String> {
    // the section's plane is the path's normal plane at its start, sketch x along world x
    let start = frame([10., 0., 0.], [0., 1., 0.], [1., 0., 0.])?;
    let path = [PathSegment::Arc { center: v3(0., 0., 0.), axis: v3(0., 0., 1.), start: v3(10., 0., 0.), angle: 350f64.to_radians() }];
    one(sweep(&start, &region(&[rect((-1., -1.), (1., 1.))])?, &path).map_err(e)?)
}
