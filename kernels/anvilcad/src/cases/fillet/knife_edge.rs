//! A wedge from z = 0 to 6 on the triangle (0, 0), (10, 0), (10, 10 tan 5°), its 5° edge along the
//! z axis rounded to radius 0.1.
use anvil_brep::Solid;

use crate::shared::{fillet, prism};

pub fn build() -> Result<Solid, String> {
    let wedge = prism(&[(0., 0.), (10., 0.), (10., 10. * 5f64.to_radians().tan())], 0., 6.)?;
    fillet(&wedge, &[[0., 0., 3.]], 0.1)
}
