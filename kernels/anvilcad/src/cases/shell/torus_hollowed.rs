//! A torus about the z axis (middle circle 6, tube 2) hollowed to a wall 0.5 thick with no opening.
use anvil_brep::Solid;

use crate::shared::{shell_open, torus};

pub fn build() -> Result<Vec<Solid>, String> {
    shell_open(&torus(&anvil_brep::Frame::world(), 6., 2.)?, &[], 0.5)
}
