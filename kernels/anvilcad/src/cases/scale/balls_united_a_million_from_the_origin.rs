//! A ball of radius 5 about (600000, −800000, 0) united with one of radius 4 about
//! (600002, −799997, 6).
use anvil_brep::Solid;

use crate::shared::{ball, union};

pub fn build() -> Result<Solid, String> {
    union(&ball([600000., -800000., 0.], 5.)?, &ball([600002., -799997., 6.], 4.)?)
}
