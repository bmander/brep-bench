//! The ring sector 8 ≤ ρ ≤ 12, |z| ≤ 1, 0° ≤ φ ≤ 60° less what a ring lying flat (a torus about the vertical through (10, 0), middle circle 1, tube 0.4) passes through, turned about
//! the z axis from −140° to 30°.
use anvil_brep::Solid;
use std::f64::consts::FRAC_PI_2;

use anvil_brep::profile::Segment;

use crate::shared::{at, cut_all, line, rect, swung, torus, turn_z, v2};

pub fn build() -> Result<Solid, String> {
    let sector = swung(rect((8., -1.), (12., 1.)).segments, 0., 60.)?;
    let tool = torus(&at([10., 0., 0.]), 1., 0.4)?;
    let fan = swung(
        vec![
            line(v2(9., -0.4), v2(11., -0.4)),
            Segment::Arc { center: v2(11., 0.), radius: 0.4, start: -FRAC_PI_2, sweep: 2. * FRAC_PI_2 },
            line(v2(11., 0.4), v2(9., 0.4)),
            Segment::Arc { center: v2(9., 0.), radius: 0.4, start: FRAC_PI_2, sweep: 2. * FRAC_PI_2 },
        ],
        -140.,
        170.,
    )?;
    // anvil-brep has no sweep of a solid. Turned about the z axis the tool fills the revolve of
    // what it covers in a half-plane through the axis, the stadium from 9 to 11 out within |z| ≤ 0.4, ended by half disks of radius 0.4, and itself where it starts and stops
    cut_all(sector, [Ok(fan), Ok(tool.transformed(&turn_z(-140.))), Ok(tool.transformed(&turn_z(30.)))])
}
