//! A ball of radius 5 centred on the origin, with a hole of radius 3 bored right through it along
//! the z axis.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const RADIUS: f64 = 5.0;
const HOLE_RADIUS: f64 = 3.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let ball = Solid::sphere(RADIUS, SEGMENTS);
    // a bore twice the ball's diameter long, its ends well clear of the poles
    let bore =
        Solid::cylinder(HOLE_RADIUS, 4.0 * RADIUS, SEGMENTS).translate(0.0, 0.0, -2.0 * RADIUS);
    Ok(ball.try_difference(&bore)?)
}
