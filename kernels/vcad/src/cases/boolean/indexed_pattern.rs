//! A disc of radius 20 from z = −5 to 0, with six holes of radius 2 through it, the first at
//! (12, 0) and each of the others the first turned 60° further about z, each hole its own cut.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const RADIUS: f64 = 20.0;
const THICKNESS: f64 = 5.0;
const HOLES: u32 = 6;
const HOLE_CIRCLE: f64 = 12.0;
const HOLE_RADIUS: f64 = 2.0;
/// How far the drill runs past each face, so its ends lie clear of the disc's.
const OVERRUN: f64 = 1.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let mut disc = Solid::cylinder(RADIUS, THICKNESS, SEGMENTS).translate(0.0, 0.0, -THICKNESS);
    let first = Solid::cylinder(HOLE_RADIUS, THICKNESS + 2.0 * OVERRUN, SEGMENTS)
        .translate(HOLE_CIRCLE, 0.0, -THICKNESS - OVERRUN);
    for k in 0..HOLES {
        // the first drill indexed k sixths of a turn about the z axis
        let drill = first.rotate(0.0, 0.0, 360.0 * k as f64 / HOLES as f64);
        disc = disc.try_difference(&drill)?;
    }
    Ok(disc)
}
