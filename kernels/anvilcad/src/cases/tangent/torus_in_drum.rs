//! A torus about the z axis (middle circle 10, tube 3) united with a drum of radius 13 on the z
//! axis from z = −1 to 1, which it touches from inside along its outer equator.
use anvil_brep::{Frame, Solid};

use crate::shared::{cylinder, torus, union};

pub fn build() -> Result<Solid, String> {
    let w = Frame::world();
    union(&torus(&w, 10., 3.)?, &cylinder(&w, 13., -1., 1.)?)
}
