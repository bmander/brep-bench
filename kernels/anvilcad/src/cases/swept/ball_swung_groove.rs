//! A block from (−6, −6, −8) to (18, 18, 0) less what a ball of radius 3 passes through, its
//! center swung a quarter turn about the z axis from (12, 0, 0) to (0, 12, 0).
use anvil_brep::Solid;
use anvil_brep::profile::Segment;

use crate::shared::{ball, block, cut_all, swung, v2};

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no sweep of a solid. A ball swung about an axis is the tube about its
    // center's arc, its disk turned, with the ball at each end
    let tube = swung(vec![Segment::circle(v2(12., 0.), 3.)], 0., 90.)?;
    cut_all(block([-6., -6., -8.], [18., 18., 0.])?, [Ok(tube), ball([12., 0., 0.], 3.), ball([0., 12., 0.], 3.)])
}
