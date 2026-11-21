//! A lemon: the segment of the disk of radius 5 about (−3, 0, 0) in the xz plane on the side
//! x ≥ 0, cut off by the z axis from (0, 0, −4) to (0, 0, 4), turned a whole turn about z.
use anvil_brep::profile::{Loop2, Segment};
use anvil_brep::{Frame, Solid};

use crate::shared::{line, turned, v2};

pub fn build() -> Result<Solid, String> {
    // in the profile's plane x is the distance from the axis and y the height
    let a = (4f64).atan2(3.);
    let segment = Loop2::new(vec![
        Segment::Arc { center: v2(-3., 0.), radius: 5., start: -a, sweep: 2. * a },
        line(v2(0., 4.), v2(0., -4.)),
    ]);
    turned(&Frame::world(), segment)
}
