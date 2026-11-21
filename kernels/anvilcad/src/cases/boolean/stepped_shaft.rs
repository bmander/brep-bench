//! A cylinder of radius 5 from z = 0 to 10 and one of radius 3 from z = 10 to 20 on it, one solid.
use anvil_brep::{Frame, Solid};

use crate::shared::{cylinder, union};

pub fn build() -> Result<Solid, String> {
    let w = Frame::world();
    union(&cylinder(&w, 5., 0., 10.)?, &cylinder(&w, 3., 10., 20.)?)
}
