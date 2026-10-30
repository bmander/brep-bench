//! A rectangular block 10 long in x, 20 in y and 30 in z, with one corner at the origin.
use vcad_kernel::Solid;

const LENGTH: f64 = 10.0;
const WIDTH: f64 = 20.0;
const HEIGHT: f64 = 30.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    // vcad's box has its corner at the origin
    Ok(Solid::cube(LENGTH, WIDTH, HEIGHT))
}
