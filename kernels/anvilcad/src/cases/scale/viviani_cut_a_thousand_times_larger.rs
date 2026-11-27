//! A ball of radius 10000 about the origin and a cylinder of radius 5000 on the line x = 5000,
//! y = 0 from z = −12000 to 12000, which touches the sphere from inside at (10000, 0, 0): the ball less the cylinder.
use anvil_brep::Solid;

use crate::shared::{ball_about, cylinder, frame, subtract};

pub fn build() -> Result<Solid, String> {
    // the ball revolved about the y axis; the cylinder's seam on the side of its wall at the z axis
    let ball = ball_about(&frame([0., 0., 0.], [0., 1., 0.], [-1., 0., 0.])?, 10000.)?;
    let rod = cylinder(&frame([5000., 0., 0.], [0., 0., 1.], [-1., 0., 0.])?, 5000., -12000., 12000.)?;
    subtract(&ball, &rod)
}
