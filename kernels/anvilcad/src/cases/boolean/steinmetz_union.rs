//! Two cylinders of radius 5 and length 30 crossing at right angles, along x and along y, each
//! from −15 to 15: their union.
use anvil_brep::Solid;

use crate::shared::{rod, union};

pub fn build() -> Result<Solid, String> {
    let a = rod([-15., 0., 0.], [15., 0., 0.], 5.)?;
    let b = rod([0., -15., 0.], [0., 15., 0.], 5.)?;
    union(&a, &b)
}
