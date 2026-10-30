//! The lens common to two balls of radius 5, one centred on the origin and one at (6, 0, 0).
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const RADIUS: f64 = 5.0;
const DISTANCE: f64 = 6.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let first = Solid::sphere(RADIUS, SEGMENTS);
    let second = Solid::sphere(RADIUS, SEGMENTS).translate(DISTANCE, 0.0, 0.0);
    Ok(first.try_intersection(&second)?)
}
