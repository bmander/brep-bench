//! A block from the origin to (10, 10, 5) whose edge at x = y = 10 is broken by a flat 0.0001
//! back; the two edges where the flat meets the sides rounded together to radius 1.
use anvil_brep::Solid;

use crate::shared::{fillet, prism};

pub fn build() -> Result<Solid, String> {
    let b = prism(&[(0., 0.), (10., 0.), (10., 9.9999), (9.9999, 10.), (0., 10.)], 0., 5.)?;
    fillet(&b, &[[10., 9.9999, 2.5], [9.9999, 10., 2.5]], 1.)
}
