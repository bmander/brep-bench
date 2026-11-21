//! The box from the origin to (10, 8, 6) shrunk by 1: every face moved 1 inward.
use anvil_brep::Solid;

use crate::shared::{block, offset};

pub fn build() -> Result<Solid, String> {
    offset(&block([0., 0., 0.], [10., 8., 6.])?, -1.)
}
