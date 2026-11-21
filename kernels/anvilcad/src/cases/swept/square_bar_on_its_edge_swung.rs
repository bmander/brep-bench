//! What an upright square bar on its corner (the square (4, 0), (6, 2), (8, 0), (6, −2), from
//! z = 0 to 3) fills as it turns a quarter turn about the z axis.
use anvil_brep::Solid;

use crate::shared::{prism, rect, swung, turn_z, union};

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no sweep of a solid. Turned about the z axis the bar fills the revolve of
    // what it covers in a half-plane through the axis, the band 4 ≤ ρ ≤ 8, 0 ≤ z ≤ 3, and itself
    // where it starts and stops
    let bar = prism(&[(4., 0.), (6., -2.), (8., 0.), (6., 2.)], 0., 3.)?;
    let band = swung(rect((4., 0.), (8., 3.)).segments, 0., 90.)?;
    union(&union(&band, &bar)?, &bar.transformed(&turn_z(90.)))
}
