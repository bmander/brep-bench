//! The lower half of the ball of radius 5 about the origin, shelled inward 1 thick, open at its flat face.
use anvil_brep::Solid;

use crate::shared::{ball, beyond, ring, shell_open, subtract};

pub fn build() -> Result<Vec<Solid>, String> {
    let h = subtract(&ball([0., 0., 0.], 5.)?, &beyond([0., 0., 0.], [0., 0., 1.], 10.)?)?;
    shell_open(&h, &[ring(2., 0.)], 1.)
}
