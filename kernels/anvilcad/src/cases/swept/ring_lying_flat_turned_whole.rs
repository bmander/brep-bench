//! What a torus whose axis is the vertical through (8, 0) (middle circle 3, tube 1) fills as it
//! turns a whole turn about the z axis.
use std::f64::consts::FRAC_PI_2;

use anvil_brep::Solid;
use anvil_brep::profile::Segment;

use crate::shared::{line, swung, v2};

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no sweep of a solid. Turned a whole turn about the z axis the ring fills the
    // revolve of what it covers in a half-plane through the axis: between 5 and 11 out within
    // |z| ≤ 1, ended by half disks of radius 1, a stadium
    swung(
        vec![
            line(v2(5., -1.), v2(11., -1.)),
            Segment::Arc { center: v2(11., 0.), radius: 1., start: -FRAC_PI_2, sweep: 2. * FRAC_PI_2 },
            line(v2(11., 1.), v2(5., 1.)),
            Segment::Arc { center: v2(5., 0.), radius: 1., start: FRAC_PI_2, sweep: 2. * FRAC_PI_2 },
        ],
        0.,
        360.,
    )
}
