//! A cylinder of radius 5 on the z axis from z = 0 to 10 capped by the half ball of radius 5 about
//! (0, 0, 10), shelled inward 1 thick, open at its hemisphere.
use std::f64::consts::FRAC_PI_2;

use anvil_brep::Solid;
use anvil_brep::profile::Segment;

use crate::shared::{line, revolved, ring, shell_open, v2};

pub fn build() -> Result<Vec<Solid>, String> {
    // the half section: the side up to z = 10, then the quarter circle of radius 5 about (0, 10)
    let capsule = revolved(vec![
        line(v2(0., 0.), v2(5., 0.)),
        line(v2(5., 0.), v2(5., 10.)),
        Segment::Arc { center: v2(0., 10.), radius: 5., start: 0., sweep: FRAC_PI_2 },
        line(v2(0., 15.), v2(0., 0.)),
    ])?;
    shell_open(&capsule, &[ring(3., 10. + 4.)], 1.)
}
