//! A frustum on the z axis from radius 6 at z = 0 to 3 at z = 6, its bottom rim rounded to radius 1.5, shelled inward 2 thick, open at its top.
use anvil_brep::Solid;

use crate::shared::{cone, fillet, ring, shell_open};

pub fn build() -> Result<Vec<Solid>, String> {
    let f = fillet(&cone(&anvil_brep::Frame::world(), 6., 3., 6.)?, &[ring(6., 0.)], 1.5)?;
    shell_open(&f, &[ring(1., 6.)], 2.)
}
