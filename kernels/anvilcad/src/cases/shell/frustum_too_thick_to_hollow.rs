//! A frustum on the z axis from radius 6 at z = 0 to 3 at z = 4, shelled inward 3.5 thick, open at its top: nothing is hollowed.
use anvil_brep::Solid;

use crate::shared::{cone, ring, shell_open};

pub fn build() -> Result<Vec<Solid>, String> {
    let f = cone(&anvil_brep::Frame::world(), 6., 3., 4.)?;
    shell_open(&f, &[ring(1., 4.)], 3.5)
}
