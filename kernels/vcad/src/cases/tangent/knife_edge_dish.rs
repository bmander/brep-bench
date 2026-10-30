//! A cylinder of radius 5 on the z axis from z = 0 to 10, less the ball of radius 5 centred on its
//! top face, whose equator is the top's rim.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const RADIUS: f64 = 5.0;
const HEIGHT: f64 = 10.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let drum = Solid::cylinder(RADIUS, HEIGHT, SEGMENTS);
    let ball = Solid::sphere(RADIUS, SEGMENTS).translate(0.0, 0.0, HEIGHT);
    Ok(drum.try_difference(&ball)?)
}
