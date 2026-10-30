//! A block from (−10, −10, −10) to (10, 10, 0), with a ball of radius 5 centred on the origin cut
//! from it: a hemispherical pocket in its top face.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const HALF_WIDTH: f64 = 10.0;
const DEPTH: f64 = 10.0;
const RADIUS: f64 = 5.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let block = Solid::cube(2.0 * HALF_WIDTH, 2.0 * HALF_WIDTH, DEPTH)
        .translate(-HALF_WIDTH, -HALF_WIDTH, -DEPTH);
    let ball = Solid::sphere(RADIUS, SEGMENTS);
    Ok(block.try_difference(&ball)?)
}
