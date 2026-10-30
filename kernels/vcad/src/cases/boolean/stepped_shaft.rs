//! A stepped shaft on the z axis: a cylinder of radius 5 from z = 0 to 10 and one of radius 3 from
//! z = 10 to 20 standing on it, made one solid.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const LOWER_RADIUS: f64 = 5.0;
const UPPER_RADIUS: f64 = 3.0;
const STEP: f64 = 10.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let lower = Solid::cylinder(LOWER_RADIUS, STEP, SEGMENTS);
    let upper = Solid::cylinder(UPPER_RADIUS, STEP, SEGMENTS).translate(0.0, 0.0, STEP);
    Ok(lower.try_union(&upper)?)
}
