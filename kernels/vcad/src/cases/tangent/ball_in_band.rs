//! A ball of radius 5 on the origin united with a cylinder of the same radius on the z axis, from
//! z = −3 to z = 3, whose wall the ball touches from inside along its equator.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const RADIUS: f64 = 5.0;
const HALF_HEIGHT: f64 = 3.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let ball = Solid::sphere(RADIUS, SEGMENTS);
    let band =
        Solid::cylinder(RADIUS, 2.0 * HALF_HEIGHT, SEGMENTS).translate(0.0, 0.0, -HALF_HEIGHT);
    Ok(ball.try_union(&band)?)
}
