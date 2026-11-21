//! A truncated cone on the z axis from z = 0 (radius 5) to z = 6 (radius 2).
use anvil_brep::{Frame, Solid};

use crate::shared::cone;

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no cone primitive: the trapezoid (0, 0), (5, 0), (2, 6), (0, 6) of a plane
    // through the z axis, revolved a full turn about it
    cone(&Frame::world(), 5., 2., 6.)
}
