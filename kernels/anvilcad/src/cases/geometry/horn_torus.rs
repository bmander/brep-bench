//! A horn torus about the z axis, its middle circle and its tube both of radius 5: the disk
//! (x − 5)² + z² ≤ 25 of the xz plane turned a whole turn about the z axis, which it touches.
use anvil_brep::{Frame, Solid};

use crate::shared::torus;

pub fn build() -> Result<Solid, String> {
    torus(&Frame::world(), 5., 5.)
}
