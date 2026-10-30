//! Two blocks 10 high on the xy plane, one spanning x and y from 0 to 10 and one from 5 to 15:
//! the second cut away from the first.
use vcad_kernel::Solid;

const SIDE: f64 = 10.0;
const HEIGHT: f64 = 10.0;
const OFFSET: f64 = 5.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let first = Solid::cube(SIDE, SIDE, HEIGHT);
    let second = Solid::cube(SIDE, SIDE, HEIGHT).translate(OFFSET, OFFSET, 0.0);
    Ok(first.try_difference(&second)?)
}
