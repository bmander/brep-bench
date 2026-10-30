//! Two cubes of side 10, one between the origin and (10, 10, 10) and one between (10, 0, 0) and
//! (20, 10, 10), joined across the face they share.
use vcad_kernel::Solid;

const SIDE: f64 = 10.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let first = Solid::cube(SIDE, SIDE, SIDE);
    let second = Solid::cube(SIDE, SIDE, SIDE).translate(SIDE, 0.0, 0.0);
    Ok(first.try_union(&second)?)
}
