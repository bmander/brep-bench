//! A torus about the z axis, centered on the origin: the tube's middle circle of radius 10, the
//! tube of radius 2.
use anvil_brep::{Frame, Solid};

use crate::shared::torus;

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no torus primitive: a circle of radius 2, 10 from the z axis in a plane
    // through it, revolved a full turn about it
    torus(&Frame::world(), 10., 2.)
}
