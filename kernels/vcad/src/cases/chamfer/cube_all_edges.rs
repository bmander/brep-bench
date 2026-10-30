//! A cube of side 10 from the origin with all twelve of its edges chamfered, distance 2 on both
//! faces of each.
use vcad_kernel::Solid;

const SIDE: f64 = 10.0;
const CHAMFER: f64 = 2.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    Ok(Solid::cube(SIDE, SIDE, SIDE).chamfer(CHAMFER)?)
}
