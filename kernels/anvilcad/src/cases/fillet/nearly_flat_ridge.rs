//! A prism from z = 0 to 6 on the pentagon (0, 0), (8, 0), (8, 5), (4, 5 + 4 tan 0.5°), (0, 5),
//! its 179° ridge rounded to radius 180.
use anvil_brep::Solid;

use crate::shared::{fillet, prism};

pub fn build() -> Result<Solid, String> {
    let top = 5. + 4. * 0.5f64.to_radians().tan();
    let roof = prism(&[(0., 0.), (8., 0.), (8., 5.), (4., top), (0., 5.)], 0., 6.)?;
    fillet(&roof, &[[4., top, 3.]], 180.)
}
