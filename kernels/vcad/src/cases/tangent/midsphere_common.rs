//! The solid common to a cube of side 10 centred on the origin and the ball on the origin of
//! radius 5√2, which touches each of the cube's edges at its midpoint.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const SIDE: f64 = 10.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let half = SIDE / 2.0;
    let cube = Solid::cube(SIDE, SIDE, SIDE).translate(-half, -half, -half);
    let ball = Solid::sphere(half * 2f64.sqrt(), SEGMENTS);
    Ok(cube.try_intersection(&ball)?)
}
