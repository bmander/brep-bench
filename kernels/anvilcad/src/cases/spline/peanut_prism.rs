//! A prism on a closed periodic cubic B-spline peanut.
use anvil_brep::{Frame, Solid, extrude};

use crate::shared::{e, periodic_bspline, region};

pub fn build() -> Result<Solid, String> {
    let peanut = periodic_bspline(
        &[
            (14., 0.), (12., 8.), (4., 6.), (0., -21. / 8.), (-4., 6.), (-12., 8.),
            (-14., 0.), (-12., -8.), (-4., -6.), (0., 21. / 8.), (4., -6.), (12., -8.),
        ],
        3,
    );
    extrude::extrude(&Frame::world(), &region(&[peanut])?, 0., 10.).map_err(e)
}
