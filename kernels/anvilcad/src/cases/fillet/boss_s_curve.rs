//! A plate from (−20, −20, 0) to (20, 20, 4) with a boss of radius 5 on the z axis from z = 4 to
//! 8, its rim at z = 8 and its root at z = 4 both rounded to radius 2.
use anvil_brep::{Frame, Solid};

use crate::shared::{block, cylinder, fillet, union};

pub fn build() -> Result<Solid, String> {
    let plate = block([-20., -20., 0.], [20., 20., 4.])?;
    let body = union(&plate, &cylinder(&Frame::world(), 5., 4., 8.)?)?;
    let rimmed = fillet(&body, &[[0., 5., 8.]], 2.)?;
    fillet(&rimmed, &[[0., 5., 4.]], 2.)
}
