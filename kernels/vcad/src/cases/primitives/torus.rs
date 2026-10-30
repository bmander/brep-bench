//! A solid torus about the z axis, centred on the origin: the circle through the middle of its
//! tube has radius 10, and the tube has radius 2.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const MAJOR_RADIUS: f64 = 10.0;
const MINOR_RADIUS: f64 = 2.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    Ok(Solid::torus(MAJOR_RADIUS, MINOR_RADIUS, SEGMENTS))
}
