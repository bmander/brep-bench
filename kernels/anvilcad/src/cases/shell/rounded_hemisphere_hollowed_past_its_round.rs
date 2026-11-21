//! The upper half of the ball of radius 5 about the origin, its rim rounded to radius 1, hollowed to a wall 2 thick with no opening.
use anvil_brep::Solid;

use crate::shared::{ball, beyond, fillet, ring, shell_open, subtract};

pub fn build() -> Result<Vec<Solid>, String> {
    let h = subtract(&ball([0., 0., 0.], 5.)?, &beyond([0., 0., 0.], [0., 0., -1.], 10.)?)?;
    let r = fillet(&h, &[ring(5., 0.)], 1.)?;
    shell_open(&r, &[], 2.)
}
