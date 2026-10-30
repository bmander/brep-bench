//! A cube of side 10 with all twelve of its edges rounded to radius 1.
use vcad_kernel::Solid;

const SIDE: f64 = 10.0;
const ROUND: f64 = 1.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    Ok(Solid::cube(SIDE, SIDE, SIDE).fillet(ROUND)?)
}
