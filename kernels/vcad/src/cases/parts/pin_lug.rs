//! A plate from (0, −5, 0) to (80, 5, 40) with a pin hole of radius 8 through its thickness along
//! y, its axis the line x = 60, z = 20.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const LENGTH: f64 = 80.0;
const THICKNESS: f64 = 10.0;
const HEIGHT: f64 = 40.0;
const HOLE_RADIUS: f64 = 8.0;
const HOLE_X: f64 = 60.0;
const HOLE_Z: f64 = 20.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let plate = Solid::cube(LENGTH, THICKNESS, HEIGHT).translate(0.0, -THICKNESS / 2.0, 0.0);
    // a cylinder along z turned −90° about x lies along +y; it runs 5 past both faces
    let hole = Solid::cylinder(HOLE_RADIUS, 2.0 * THICKNESS, SEGMENTS)
        .rotate(-90.0, 0.0, 0.0)
        .translate(HOLE_X, -THICKNESS, HOLE_Z);
    Ok(plate.try_difference(&hole)?)
}
