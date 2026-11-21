//! A plate from (−10, −10, 0) to (10, 10, 2) with a hole of radius 3 on the z axis, both of the
//! hole's rims rounded to radius 1.
use anvil_brep::{Frame, Solid};

use crate::shared::{block, cylinder, fillet, ring, subtract};

pub fn build() -> Result<Solid, String> {
    let plate = subtract(&block([-10., -10., 0.], [10., 10., 2.])?, &cylinder(&Frame::world(), 3., -1., 3.)?)?;
    fillet(&plate, &[ring(3., 2.), ring(3., 0.)], 1.)
}
