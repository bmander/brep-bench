//! A frustum on the z axis from radius 6 at z = 0 to 3 at z = 6, both rims rounded to radius 1.5, hollowed to a wall 2 thick with no opening.
use anvil_brep::Solid;

use crate::shared::{cone, fillet, ring, shell_open};

pub fn build() -> Result<Vec<Solid>, String> {
    let f = fillet(&cone(&anvil_brep::Frame::world(), 6., 3., 6.)?, &[ring(6., 0.), ring(3., 6.)], 1.5)?;
    shell_open(&f, &[], 2.)
}
