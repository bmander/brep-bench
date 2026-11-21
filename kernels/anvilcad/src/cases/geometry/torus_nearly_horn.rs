//! A torus about the z axis whose middle circle has radius 5 and whose tube has radius 4.99.
use anvil_brep::{Frame, Solid};

use crate::shared::torus;

pub fn build() -> Result<Solid, String> {
    torus(&Frame::world(), 5., 4.99)
}
