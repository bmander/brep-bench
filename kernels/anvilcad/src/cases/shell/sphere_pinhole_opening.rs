//! The ball of radius 5 about the origin below z = 3.9999, shelled inward 1 thick, open at its flat face, which the moved sphere meets in a pinhole.
use anvil_brep::Solid;

use crate::shared::{ball, beyond, shell_open, subtract};

pub fn build() -> Result<Vec<Solid>, String> {
    let b = subtract(&ball([0., 0., 0.], 5.)?, &beyond([0., 0., 3.9999], [0., 0., 1.], 10.)?)?;
    shell_open(&b, &[[0., 0., 3.9999]], 1.)
}
