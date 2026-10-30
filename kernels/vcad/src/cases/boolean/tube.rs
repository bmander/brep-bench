//! A cylinder of radius 5 on the z axis from z = 0 to 10, with a hole of radius 3 bored along its
//! axis.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const RADIUS: f64 = 5.0;
const HOLE_RADIUS: f64 = 3.0;
const HEIGHT: f64 = 10.0;
/// How far the drill runs past each end, as the case asks.
const OVERRUN: f64 = 1.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let cylinder = Solid::cylinder(RADIUS, HEIGHT, SEGMENTS);
    let drill = Solid::cylinder(HOLE_RADIUS, HEIGHT + 2.0 * OVERRUN, SEGMENTS)
        .translate(0.0, 0.0, -OVERRUN);
    Ok(cylinder.try_difference(&drill)?)
}
