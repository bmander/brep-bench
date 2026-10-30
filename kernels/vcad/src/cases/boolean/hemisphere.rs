//! A ball of radius 5 centred on the origin, with everything below the xy plane cut away.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const RADIUS: f64 = 5.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let ball = Solid::sphere(RADIUS, SEGMENTS);
    // a block filling z < 0 around the ball, its top the xy plane, the rest well clear of it
    let reach = RADIUS + 1.0;
    let below = Solid::cube(2.0 * reach, 2.0 * reach, reach).translate(-reach, -reach, -reach);
    Ok(ball.try_difference(&below)?)
}
