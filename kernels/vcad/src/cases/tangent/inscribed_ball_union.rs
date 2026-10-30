//! A cube of side 10 between the origin and (10, 10, 10) united with its inscribed ball, of radius
//! 5 at (5, 5, 5): the cube.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const SIDE: f64 = 10.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let cube = Solid::cube(SIDE, SIDE, SIDE);
    let ball = Solid::sphere(SIDE / 2.0, SEGMENTS).translate(SIDE / 2.0, SIDE / 2.0, SIDE / 2.0);
    Ok(cube.try_union(&ball)?)
}
