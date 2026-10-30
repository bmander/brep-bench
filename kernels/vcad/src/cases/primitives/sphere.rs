//! A solid ball of radius 5 centred on the origin.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const RADIUS: f64 = 5.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    Ok(Solid::sphere(RADIUS, SEGMENTS))
}
