//! A ball of radius 10 about the origin and a cylinder of radius 5 on the line x = 5, y = 0 from
//! z = −12 to 12, which touches the sphere from inside at (10, 0, 0): the ball less the cylinder.
use anvil_brep::Solid;

use crate::shared::{ball_about, cylinder, frame, subtract};

pub fn build() -> Result<Solid, String> {
    // the ball revolved about the y axis; the cylinder's seam on the side of its wall at the z axis
    let ball = ball_about(&frame([0., 0., 0.], [0., 1., 0.], [-1., 0., 0.])?, 10.)?;
    let rod = cylinder(&frame([5., 0., 0.], [0., 0., 1.], [-1., 0., 0.])?, 5., -12., 12.)?;
    subtract(&ball, &rod)
}
