//! A block from (−10, −10, 0) to (10, 10, 10) united with a solid cone on the z axis, its apex
//! at the origin, the middle of the block's floor, and its base the disc of radius 5 at z = 15.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const HALF_WIDTH: f64 = 10.0;
const BLOCK_HEIGHT: f64 = 10.0;
const CONE_RADIUS: f64 = 5.0;
const CONE_HEIGHT: f64 = 15.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let block = Solid::cube(2.0 * HALF_WIDTH, 2.0 * HALF_WIDTH, BLOCK_HEIGHT)
        .translate(-HALF_WIDTH, -HALF_WIDTH, 0.0);
    // vcad's pointed cone has its base on z = 0 and its apex above it: turned over about x, its
    // apex is at z = −15, then lifted so the apex is at the origin and the base at z = 15
    let cone = Solid::cone(CONE_RADIUS, 0.0, CONE_HEIGHT, SEGMENTS)
        .rotate(180.0, 0.0, 0.0)
        .translate(0.0, 0.0, CONE_HEIGHT);
    Ok(block.try_union(&cone)?)
}
