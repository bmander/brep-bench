//! A frustum on the z axis from radius 18.6 at z = 0 to 9.3 at z = 12.4, bored through by a hole of
//! radius 3.1, shelled inward 3.1 thick, open at its top: the cavity closes below it.
use anvil_brep::Solid;

use crate::shared::{cone, cylinder, ring, shell_open, subtract};

pub fn build() -> Result<Vec<Solid>, String> {
    let w = anvil_brep::Frame::world();
    let f = subtract(&cone(&w, 18.6, 9.3, 12.4)?, &cylinder(&w, 3.1, -3.1, 15.5)?)?;
    shell_open(&f, &[ring(6.2, 12.4)], 3.1)
}
