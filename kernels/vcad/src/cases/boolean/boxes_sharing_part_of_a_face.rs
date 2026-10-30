//! A cube of side 10 between the origin and (10, 10, 10), joined to a second between (5, 0, 10)
//! and (15, 10, 20) standing on half of its top.
use vcad_kernel::Solid;

const SIDE: f64 = 10.0;
const OVERHANG: f64 = 5.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let first = Solid::cube(SIDE, SIDE, SIDE);
    let second = Solid::cube(SIDE, SIDE, SIDE).translate(OVERHANG, 0.0, SIDE);
    Ok(first.try_union(&second)?)
}
