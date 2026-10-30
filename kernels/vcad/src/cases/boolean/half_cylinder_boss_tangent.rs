//! A cube of side 10 between the origin and (10, 10, 10), united with a cylinder of radius 5 and
//! height 10 standing on the xy plane with its axis at x = 10, y = 5: a half-round end tangent to
//! the cube's faces y = 0 and y = 10.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const SIDE: f64 = 10.0;
const RADIUS: f64 = 5.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let cube = Solid::cube(SIDE, SIDE, SIDE);
    let boss = Solid::cylinder(RADIUS, SIDE, SEGMENTS).translate(SIDE, SIDE / 2.0, 0.0);
    Ok(cube.try_union(&boss)?)
}
