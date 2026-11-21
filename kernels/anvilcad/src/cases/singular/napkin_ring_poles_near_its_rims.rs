//! A ball of radius 5 about the origin bored through along z by a hole of radius 3, the ball's
//! frame along (3.1, 0, 4).
use anvil_brep::{Frame, Solid};

use crate::shared::{ball_about, cylinder, frame, subtract};

pub fn build() -> Result<Solid, String> {
    // the ball revolved about the line through the origin along (3.1, 0, 4) (anvil-brep's spheres have no
    // poles, but this is where they would be)
    let ball = ball_about(&frame([0., 0., 0.], [3.1, 0., 4.], [0., 1., 0.])?, 5.)?;
    // the drill runs past the ball at both ends
    subtract(&ball, &cylinder(&Frame::world(), 3., -6., 6.)?)
}
