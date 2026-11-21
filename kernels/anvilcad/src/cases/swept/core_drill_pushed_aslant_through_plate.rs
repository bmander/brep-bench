//! A plate from (−10, −10, 0) to (10, 10, 4) less what a core drill (a tube from 1.5 to 2.5 about
//! the line through (0, 0, 2) along (sin 30°, 0, cos 30°)) passes through, pushed 24 along its
//! axis from below the plate to above it: the plate and the core it cuts free.
use anvil_brep::Solid;
use anvil_brep::boolean::{BoolOp, boolean};

use anvil_brep::extrude;

use crate::shared::{block, circle, e, frame, region};

pub fn build() -> Result<Vec<Solid>, String> {
    // anvil-brep has no sweep of a solid. Pushed along its own axis the tube fills itself
    // lengthened by the push: from 18 before (0, 0, 2) to 18 past it
    let (s, c) = 30f64.to_radians().sin_cos();
    let axis = frame([0., 0., 2.], [s, 0., c], [0., 1., 0.])?;
    let ring = region(&[circle((0., 0.), 2.5), circle((0., 0.), 1.5)])?;
    let tube = extrude::extrude(&axis, &ring, -18., 18.).map_err(e)?;
    boolean(&block([-10., -10., 0.], [10., 10., 4.])?, &tube, BoolOp::Subtract).map_err(e)
}
