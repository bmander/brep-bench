//! Two cylinders of radius 5 on the z axis, from z = 0 to 10 and from z = 6 to 16, united: their
//! walls coincide between z = 6 and z = 10.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const RADIUS: f64 = 5.0;
const HEIGHT: f64 = 10.0;
const SECOND_BASE: f64 = 6.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let first = Solid::cylinder(RADIUS, HEIGHT, SEGMENTS);
    // vcad's cylinder has its seam on +x: the second is turned a quarter turn so the seams differ
    let second = Solid::cylinder(RADIUS, HEIGHT, SEGMENTS)
        .rotate(0.0, 0.0, 90.0)
        .translate(0.0, 0.0, SECOND_BASE);
    Ok(first.try_union(&second)?)
}
