//! A plate from the origin to (100, 20, 10), drilled through along z by nine holes of radius 2 at
//! y = 10 and x = 10, 20, … 90, each hole its own cut.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const LENGTH: f64 = 100.0;
const WIDTH: f64 = 20.0;
const THICKNESS: f64 = 10.0;
const HOLES: u32 = 9;
const PITCH: f64 = 10.0;
const HOLE_RADIUS: f64 = 2.0;
/// How far each drill runs past the plate's faces, so its ends lie clear of them.
const OVERRUN: f64 = 1.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let mut plate = Solid::cube(LENGTH, WIDTH, THICKNESS);
    let drill = Solid::cylinder(HOLE_RADIUS, THICKNESS + 2.0 * OVERRUN, SEGMENTS);
    for k in 1..=HOLES {
        let hole = drill.translate(PITCH * k as f64, WIDTH / 2.0, -OVERRUN);
        plate = plate.try_difference(&hole)?;
    }
    Ok(plate)
}
