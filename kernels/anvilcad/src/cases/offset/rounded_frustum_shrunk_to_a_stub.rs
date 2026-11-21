//! A frustum on the z axis from radius 6 at z = 0 to 3 at z = 6, both rims rounded to radius 1.5, shrunk
//! by 2.5.
use anvil_brep::{Frame, Solid};

use crate::shared::{cone, fillet, offset, ring};

pub fn build() -> Result<Solid, String> {
    let f = fillet(&cone(&Frame::world(), 6., 3., 6.)?, &[ring(6., 0.), ring(3., 6.)], 1.5)?;
    offset(&f, -2.5)
}
