//! The common part of two cylinders of radius 0.003 and length 0.02 crossing square at the
//! origin, along x and along y.
use anvil_brep::Solid;

use crate::shared::{intersect, rod};

pub fn build() -> Result<Solid, String> {
    intersect(&rod([-0.01, 0., 0.], [0.01, 0., 0.], 0.003)?, &rod([0., -0.01, 0.], [0., 0.01, 0.], 0.003)?)
}
