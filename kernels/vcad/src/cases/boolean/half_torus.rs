//! A torus about the z axis, centred on the origin, of radii 10 and 2, with everything where
//! x < 0 cut away.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const MAJOR_RADIUS: f64 = 10.0;
const MINOR_RADIUS: f64 = 2.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let torus = Solid::torus(MAJOR_RADIUS, MINOR_RADIUS, SEGMENTS);
    // a block filling x < 0 around the torus, its face x = 0 the cut, the rest well clear of it
    let reach = MAJOR_RADIUS + MINOR_RADIUS + 1.0;
    let half_space = Solid::cube(reach, 2.0 * reach, 2.0 * reach).translate(-reach, -reach, -reach);
    Ok(torus.try_difference(&half_space)?)
}
