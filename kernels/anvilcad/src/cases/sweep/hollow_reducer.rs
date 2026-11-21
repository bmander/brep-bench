//! A hollow square tube tapering along y from y = 0 (24 × 24 outside, a 20 × 20 hole) to y = 40
//! (12 × 12 outside, an 8 × 8 hole), its corners straight lines.
use anvil_brep::Solid;
use anvil_brep::loft::{Section, loft_ruled};

use crate::shared::{e, frame, rect, subtract};

pub fn build() -> Result<Solid, String> {
    // anvil-brep lofts a single loop, not a section with a hole: the outside and the hole are each
    // a ruled loft between their squares, and the tube is the one less the other
    let end = |y: f64| frame([0., y, 0.], [0., 1., 0.], [1., 0., 0.]);
    let square = |y: f64, h: f64| -> Result<Section, String> { Ok(Section::Loop { frame: end(y)?, outline: rect((-h, -h), (h, h)) }) };
    let outside = loft_ruled(&square(0., 12.)?, &square(40., 6.)?, [None, None]).map_err(e)?;
    // the hole's loft runs past both ends, keeping its taper
    let hole = loft_ruled(&square(-1., 10.15)?, &square(41., 3.85)?, [None, None]).map_err(e)?;
    subtract(&outside, &hole)
}
