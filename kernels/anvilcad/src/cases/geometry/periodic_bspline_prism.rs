//! A prism on a closed periodic quadratic B-spline.
use anvil_brep::{Frame, Solid, extrude};

use crate::shared::{e, periodic_bspline, region};

pub fn build() -> Result<Solid, String> {
    // four parabolic arcs, each a quadratic span raised to a cubic one
    let outline = periodic_bspline(&[(10., 10.), (-10., 10.), (-10., -10.), (10., -10.)], 2);
    extrude::extrude(&Frame::world(), &region(&[outline])?, 0., 10.).map_err(e)
}
