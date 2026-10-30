//! A cube of side 10 between the origin and (10, 10, 10) with a slot 10⁻⁶ wide, between x = 5
//! and x = 5.000001, cut through it in y from z = 5 up through the top.
use vcad_kernel::Solid;

const SIDE: f64 = 10.0;
const SLOT_X: f64 = 5.0;
const SLOT_WIDTH: f64 = 1e-6;
const SLOT_FLOOR: f64 = 5.0;
/// How far the cutter runs past the cube's front, back and top.
const OVERRUN: f64 = 1.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let cube = Solid::cube(SIDE, SIDE, SIDE);
    let saw = Solid::cube(SLOT_WIDTH, SIDE + 2.0 * OVERRUN, SIDE - SLOT_FLOOR + OVERRUN)
        .translate(SLOT_X, -OVERRUN, SLOT_FLOOR);
    Ok(cube.try_difference(&saw)?)
}
