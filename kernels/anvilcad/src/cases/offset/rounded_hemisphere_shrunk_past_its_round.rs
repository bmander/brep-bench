//! The upper half of the ball of radius 5 about the origin, its rim rounded to radius 1, shrunk by
//! 3/2.
use anvil_brep::Solid;

use crate::shared::{ball, beyond, fillet, offset, ring, subtract};

pub fn build() -> Result<Solid, String> {
    let h = subtract(&ball([0., 0., 0.], 5.)?, &beyond([0., 0., 0.], [0., 0., -1.], 10.)?)?;
    offset(&fillet(&h, &[ring(5., 0.)], 1.)?, -1.5)
}
