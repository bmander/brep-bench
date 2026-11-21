//! A cone of base radius 5 at z = 0 and apex at (0, 0, 10), less the cone of base radius 4.999
//! with the same base plane and apex.
use anvil_brep::{Frame, Solid};

use crate::shared::{cone, subtract};

pub fn build() -> Result<Solid, String> {
    let w = Frame::world();
    subtract(&cone(&w, 5., 0., 10.)?, &cone(&w, 4.999, 0., 10.)?)
}
