//! A cone on the z axis from radius 5 at z = 0 to its apex at (0, 0, 10), everything on the side
//! x < 0 cut away.
use anvil_brep::{Frame, Solid};

use crate::shared::{beyond, cone, subtract};

pub fn build() -> Result<Solid, String> {
    subtract(&cone(&Frame::world(), 5., 0., 10.)?, &beyond([0., 0., 0.], [-1., 0., 0.], 20.)?)
}
