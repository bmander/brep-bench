//! A block from the origin to (10, 4, 6), the two top edges along x chamfered 2 on both faces,
//! half the top's width: the top becomes a ridge.
use anvil_brep::Solid;

use crate::shared::{block, chamfer};

pub fn build() -> Result<Solid, String> {
    chamfer(&block([0., 0., 0.], [10., 4., 6.])?, &[[5., 0., 6.], [5., 4., 6.]], 2.)
}
