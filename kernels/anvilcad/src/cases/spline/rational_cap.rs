//! The region of the xz plane between the z axis from (0, 0, −5) to (0, 0, 10), the segment from
//! (0, 0, −5) to (5√3, 0, −5), and the rational quadratic Bézier from (5√3, 0, −5) to (0, 0, 10)
//! with weights 1, ½, 1 (the arc of the circle of radius 10 about the origin), turned a full turn
//! about z.
use std::f64::consts::FRAC_PI_6;

use anvil_brep::Solid;
use anvil_brep::profile::Segment;

use crate::shared::{line, revolved, v2};

pub fn build() -> Result<Solid, String> {
    // anvil-brep's profiles hold no rational curve, but this one is exactly a circular arc: from
    // −30° round 120° to the top of the circle of radius 10 about the origin
    let s3 = 3f64.sqrt();
    revolved(vec![
        line(v2(0., -5.), v2(5. * s3, -5.)),
        Segment::Arc { center: v2(0., 0.), radius: 10., start: -FRAC_PI_6, sweep: 4. * FRAC_PI_6 },
        line(v2(0., 10.), v2(0., -5.)),
    ])
}
