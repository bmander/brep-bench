//! A ball of radius 20 centred on the origin, with a hole of radius 6 bored right through it
//! parallel to the y axis, its axis the line x = 5, z = 0.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const RADIUS: f64 = 20.0;
const HOLE_RADIUS: f64 = 6.0;
const HOLE_X: f64 = 5.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let ball = Solid::sphere(RADIUS, SEGMENTS);
    // a bore longer than the ball's diameter, centred on the xy plane, then turned onto y
    let length = 2.0 * RADIUS + 10.0;
    let bore = Solid::cylinder(HOLE_RADIUS, length, SEGMENTS)
        .translate(0.0, 0.0, -length / 2.0)
        .rotate(90.0, 0.0, 0.0)
        .translate(HOLE_X, 0.0, 0.0);
    Ok(ball.try_difference(&bore)?)
}
