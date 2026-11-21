//! A stepped shaft on the z axis: radius 20 from z = 0 to 60 and radius 12 from 60 to 160, the
//! inside corner at the shoulder rounded to radius 4.
use std::f64::consts::FRAC_PI_2;

use anvil_brep::Solid;
use anvil_brep::profile::Segment;

use crate::shared::{line, revolved, v2};

pub fn build() -> Result<Solid, String> {
    revolved(vec![
        line(v2(0., 0.), v2(20., 0.)),
        line(v2(20., 0.), v2(20., 60.)),
        line(v2(20., 60.), v2(16., 60.)),
        Segment::Arc { center: v2(16., 64.), radius: 4., start: -FRAC_PI_2, sweep: -FRAC_PI_2 },
        line(v2(12., 64.), v2(12., 160.)),
        line(v2(12., 160.), v2(0., 160.)),
        line(v2(0., 160.), v2(0., 0.)),
    ])
}
