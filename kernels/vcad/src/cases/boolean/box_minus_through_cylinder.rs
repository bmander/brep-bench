//! A cube of side 10 between the origin and (10, 10, 10), drilled through along z by a hole of
//! radius 3 centred at x = 5, y = 5.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const SIDE: f64 = 10.0;
const HOLE_RADIUS: f64 = 3.0;
/// How far the drill runs past each face of the cube, as the case asks.
const OVERRUN: f64 = 1.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let cube = Solid::cube(SIDE, SIDE, SIDE);
    let drill = Solid::cylinder(HOLE_RADIUS, SIDE + 2.0 * OVERRUN, SEGMENTS)
        .translate(SIDE / 2.0, SIDE / 2.0, -OVERRUN);
    Ok(cube.try_difference(&drill)?)
}
