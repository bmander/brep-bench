//! A solid cylinder of radius 5 and height 10 on the z axis, from z = 0 to z = 10.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const RADIUS: f64 = 5.0;
const HEIGHT: f64 = 10.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    // vcad's cylinder stands on the xy plane about the z axis
    Ok(Solid::cylinder(RADIUS, HEIGHT, SEGMENTS))
}
