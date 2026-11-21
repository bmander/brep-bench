//! A torus about the z axis (middle circle 10, tube 3) shrunk by 1.5.
use anvil_brep::{Frame, Solid};

use crate::shared::{offset, torus};

pub fn build() -> Result<Solid, String> {
    offset(&torus(&Frame::world(), 10., 3.)?, -1.5)
}
