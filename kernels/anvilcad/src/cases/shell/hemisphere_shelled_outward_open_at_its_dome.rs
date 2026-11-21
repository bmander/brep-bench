//! The upper half of the ball of radius 5 about the origin, shelled outward 1. thick, open at its dome: the slice of the ball under its flat face.
use anvil_brep::Solid;

use crate::shared::{ball, beyond, shell_outward, subtract};

pub fn build() -> Result<Vec<Solid>, String> {
    let h = subtract(&ball([0., 0., 0.], 5.)?, &beyond([0., 0., 0.], [0., 0., -1.], 10.)?)?;
    shell_outward(&h, &[[0., 0., 5.]], 1.)
}
