//! A cone of base radius 0.0007575 at z = 0 and apex 0.0015 above it, less the cone of base
//! radius 0.00075 with the same base plane and apex.
use anvil_brep::{Frame, Solid};

use crate::shared::{cone, subtract};

pub fn build() -> Result<Solid, String> {
    let w = Frame::world();
    subtract(&cone(&w, 0.0007575, 0., 0.0015)?, &cone(&w, 0.00075, 0., 0.0015)?)
}
