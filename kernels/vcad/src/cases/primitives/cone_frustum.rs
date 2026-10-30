//! A truncated cone on the z axis, from z = 0 to z = 6, of radius 5 at its base and 2 at its top.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const BASE_RADIUS: f64 = 5.0;
const TOP_RADIUS: f64 = 2.0;
const HEIGHT: f64 = 6.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    Ok(Solid::cone(BASE_RADIUS, TOP_RADIUS, HEIGHT, SEGMENTS))
}
