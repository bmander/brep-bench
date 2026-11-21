//! The halves z ≥ 0 and x ≥ 0 of the ball of radius 5 about the origin, united: their curved faces
//! lie on one sphere and overlap on a quarter of it.
use anvil_brep::Solid;

use crate::shared::{ball_about, beyond, frame, subtract, union};

pub fn build() -> Result<Solid, String> {
    // each half revolved about the axis square to its flat face
    let up = ball_about(&frame([0., 0., 0.], [0., 0., 1.], [1., 0., 0.])?, 5.)?;
    let side = ball_about(&frame([0., 0., 0.], [1., 0., 0.], [0., 1., 0.])?, 5.)?;
    let top = subtract(&up, &beyond([0., 0., 0.], [0., 0., -1.], 10.)?)?;
    let right = subtract(&side, &beyond([0., 0., 0.], [-1., 0., 0.], 10.)?)?;
    union(&top, &right)
}
