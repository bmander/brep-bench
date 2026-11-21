//! A spindle torus about the z axis, middle circle 3 and tube 5: the part x ≥ 0 of the disk
//! (x − 3)² + z² ≤ 25 of the xz plane, bounded by the tube's outer arc and the z axis from
//! (0, 0, −4) to (0, 0, 4), turned a whole turn about z.
use std::f64::consts::PI;

use anvil_brep::profile::{Loop2, Segment};
use anvil_brep::{Frame, Solid};

use crate::shared::{line, turned, v2};

pub fn build() -> Result<Solid, String> {
    // the outer arc of the tube's circle, from (0, −4) round through (8, 0) to (0, 4)
    let a = PI - (4f64).atan2(3.);
    let apple = Loop2::new(vec![
        Segment::Arc { center: v2(3., 0.), radius: 5., start: -a, sweep: 2. * a },
        line(v2(0., 4.), v2(0., -4.)),
    ]);
    turned(&Frame::world(), apple)
}
