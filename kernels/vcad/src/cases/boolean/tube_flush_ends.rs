//! A cylinder of radius 5 on the z axis from z = 0 to 10, with a cylinder of radius 3 on the same
//! axis and over exactly the same span cut from it: the cutter's ends lie in the first's.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const RADIUS: f64 = 5.0;
const HOLE_RADIUS: f64 = 3.0;
const HEIGHT: f64 = 10.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let cylinder = Solid::cylinder(RADIUS, HEIGHT, SEGMENTS);
    let core = Solid::cylinder(HOLE_RADIUS, HEIGHT, SEGMENTS);
    Ok(cylinder.try_difference(&core)?)
}
