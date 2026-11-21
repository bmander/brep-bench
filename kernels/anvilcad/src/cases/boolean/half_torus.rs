//! A torus about the z axis (middle circle 10, tube 2), everything where x < 0 cut away.
use anvil_brep::{Frame, Solid};

use crate::shared::{block, subtract, torus};

pub fn build() -> Result<Solid, String> {
    let ring = torus(&Frame::world(), 10., 2.)?;
    subtract(&ring, &block([-13., -13., -3.], [0., 13., 3.])?)
}
