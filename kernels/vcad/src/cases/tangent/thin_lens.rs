//! The lens common to two balls of radius 5, one on the origin and one at (9.999999, 0, 0), which
//! overlap to a depth of 10⁻⁶.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const RADIUS: f64 = 5.0;
const SPACING: f64 = 9.999999;

pub fn build() -> Result<Solid, crate::Refusal> {
    let first = Solid::sphere(RADIUS, SEGMENTS);
    let second = Solid::sphere(RADIUS, SEGMENTS).translate(SPACING, 0.0, 0.0);
    Ok(first.try_intersection(&second)?)
}
