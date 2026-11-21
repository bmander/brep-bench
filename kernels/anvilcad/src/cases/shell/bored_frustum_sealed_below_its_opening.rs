//! A frustum on the z axis from radius 6 at z = 0 to 3 at z = 4, bored through by a hole of radius 1, shelled inward 1 thick, open at its top: the cavity closes below it.
use anvil_brep::Solid;

use crate::shared::{cone, cylinder, ring, shell_open, subtract};

pub fn build() -> Result<Vec<Solid>, String> {
    let w = anvil_brep::Frame::world();
    let f = subtract(&cone(&w, 6., 3., 4.)?, &cylinder(&w, 1., -1., 5.)?)?;
    shell_open(&f, &[ring(2., 4.)], 1.)
}
