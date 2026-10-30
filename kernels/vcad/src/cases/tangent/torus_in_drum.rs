//! A torus about the z axis, its tube's middle circle of radius 10 and the tube of radius 3,
//! united with a drum of radius 13 on the z axis from z = −1 to 1, whose wall touches the torus's
//! outer equator.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const MAJOR_RADIUS: f64 = 10.0;
const MINOR_RADIUS: f64 = 3.0;
const DRUM_RADIUS: f64 = 13.0;
const DRUM_HALF_HEIGHT: f64 = 1.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let torus = Solid::torus(MAJOR_RADIUS, MINOR_RADIUS, SEGMENTS);
    let drum = Solid::cylinder(DRUM_RADIUS, 2.0 * DRUM_HALF_HEIGHT, SEGMENTS)
        .translate(0.0, 0.0, -DRUM_HALF_HEIGHT);
    Ok(torus.try_union(&drum)?)
}
