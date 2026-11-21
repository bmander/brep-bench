//! A cone on the z axis from radius 6 at z = 0 to its apex at z = 12, its tip rounded by a ball of
//! radius 1 tangent to its side, shrunk by 2..
use std::f64::consts::FRAC_PI_2;

use anvil_brep::Solid;
use anvil_brep::profile::Segment;

use crate::shared::{line, offset, revolved, v2};

pub fn build() -> Result<Solid, String> {
    // the half section: the base, the side up to where the tip ball of radius 1 about
    // (0, 12 − √5) touches it, that ball's arc to the axis, and down the axis
    let s5 = 5f64.sqrt();
    let c = 12. - s5;
    let touch = v2(2. / s5, 12. - 4. / s5);
    let start = (touch.y - c).atan2(touch.x);
    let cone = revolved(vec![
        line(v2(0., 0.), v2(6., 0.)),
        line(v2(6., 0.), touch),
        Segment::Arc { center: v2(0., c), radius: 1., start, sweep: FRAC_PI_2 - start },
        line(v2(0., c + 1.), v2(0., 0.)),
    ])?;
    offset(&cone, -2.)
}
