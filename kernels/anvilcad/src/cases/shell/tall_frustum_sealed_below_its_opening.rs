//! A frustum on the z axis from radius 7 at z = 0 to 1 at z = 8, shelled inward 2 thick, open at its top: the cavity closes below it.
use anvil_brep::Solid;

use crate::shared::{cone, ring, shell_open};

pub fn build() -> Result<Vec<Solid>, String> {
    let f = cone(&anvil_brep::Frame::world(), 7., 1., 8.)?;
    shell_open(&f, &[ring(0.5, 8.)], 2.)
}
