//! Balls of radius 5, 4 and 3 about the origin, (7, 0, 0) and (10, 4, 0), united one after
//! another.
use anvil_brep::Solid;

use crate::shared::{ball, union};

pub fn build() -> Result<Solid, String> {
    let ab = union(&ball([0., 0., 0.], 5.)?, &ball([7., 0., 0.], 4.)?)?;
    union(&ab, &ball([10., 4., 0.], 3.)?)
}
