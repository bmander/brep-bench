//! A cone on the z axis from radius 6 at z = 0 to its apex at (0, 0, 8), shelled inward 1 thick, open at its base.
use anvil_brep::Solid;

use crate::shared::{cone, ring, shell_open};

pub fn build() -> Result<Vec<Solid>, String> {
    let c = cone(&anvil_brep::Frame::world(), 6., 0., 8.)?;
    shell_open(&c, &[ring(2., 0.)], 1.)
}
