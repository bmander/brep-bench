//! A ball of radius 15 on the origin united with a solid cone on the z axis, its apex at
//! (0, 0, −25) and its base the disc of radius 12 at z = −9, its side tangent to the sphere
//! there.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const BALL_RADIUS: f64 = 15.0;
const APEX_Z: f64 = -25.0;
const BASE_Z: f64 = -9.0;
const BASE_RADIUS: f64 = 12.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let scoop = Solid::sphere(BALL_RADIUS, SEGMENTS);
    // vcad's pointed cone has its base on z = 0 and its apex above it: turned over about x, its
    // apex is below, and lowered so the base is at z = −9 and the apex at z = −25
    let cone = Solid::cone(BASE_RADIUS, 0.0, BASE_Z - APEX_Z, SEGMENTS)
        .rotate(180.0, 0.0, 0.0)
        .translate(0.0, 0.0, BASE_Z);
    Ok(scoop.try_union(&cone)?)
}
