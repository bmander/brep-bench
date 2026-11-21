//! The halves z ≥ 0 and x ≥ 0 of the torus about the z axis (middle circle 10, tube 3), united:
//! their curved faces lie on one torus and overlap on a quarter of it.
use anvil_brep::Solid;

use crate::shared::{beyond, frame, subtract, torus, union};

pub fn build() -> Result<Solid, String> {
    let first = torus(&frame([0., 0., 0.], [0., 0., 1.], [1., 0., 0.])?, 10., 3.)?;
    // the second turned a quarter turn about z, so that its seam is not the first's
    let second = torus(&frame([0., 0., 0.], [0., 0., 1.], [0., 1., 0.])?, 10., 3.)?;
    let top = subtract(&first, &beyond([0., 0., 0.], [0., 0., -1.], 20.)?)?;
    let right = subtract(&second, &beyond([0., 0., 0.], [-1., 0., 0.], 20.)?)?;
    union(&top, &right)
}
