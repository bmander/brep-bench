//! The ring sector 8 ≤ ρ ≤ 12, |z| ≤ 1, 0° ≤ φ ≤ 60° less what half a drum (the half y ≤ 0 of radius 1 about the vertical through (10, 0), from z = −0.5 to 0.5) passes through, turned about
//! the z axis from −140° to 30°.
use anvil_brep::Solid;

use crate::shared::{at, block, cut_all, cylinder, rect, subtract, swung, turn_z};

pub fn build() -> Result<Solid, String> {
    let sector = swung(rect((8., -1.), (12., 1.)).segments, 0., 60.)?;
    let tool = subtract(&cylinder(&at([10., 0., 0.]), 1., -0.5, 0.5)?, &block([8., 0., -1.], [12., 2., 1.])?)?;
    let fan = swung(rect((9., -0.5), (11., 0.5)).segments, -140., 170.)?;
    // anvil-brep has no sweep of a solid. Turned about the z axis the tool fills the revolve of
    // what it covers in a half-plane through the axis, from 9 to 11 out within |z| ≤ 0.5, and itself where it starts and stops
    cut_all(sector, [Ok(fan), Ok(tool.transformed(&turn_z(-140.))), Ok(tool.transformed(&turn_z(30.)))])
}
