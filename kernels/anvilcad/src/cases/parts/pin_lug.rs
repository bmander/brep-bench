//! A lug from (0, −5, 0) to (80, 5, 40) with a pin hole of radius 8 through it along y, on the
//! line x = 60, z = 20.
use anvil_brep::Solid;

use crate::shared::{block, rod, subtract};

pub fn build() -> Result<Solid, String> {
    subtract(&block([0., -5., 0.], [80., 5., 40.])?, &rod([60., -6., 20.], [60., 6., 20.], 8.)?)
}
