//! A plate from the origin to (100, 20, 10), drilled through along z by nine holes of radius 2 at
//! y = 10 and x = 10, 20, … 90, each hole its own cut.
use anvil_brep::Solid;

use crate::shared::{at, block, cut_all, cylinder};

pub fn build() -> Result<Solid, String> {
    let plate = block([0., 0., 0.], [100., 20., 10.])?;
    // each drill runs past both faces
    cut_all(plate, (1..=9).map(|k| cylinder(&at([10. * k as f64, 10., 0.]), 2., -1., 11.)))
}
