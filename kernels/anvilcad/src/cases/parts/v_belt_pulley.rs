//! A pulley on the z axis: a hub of radius 12 from z = −14 to 14, a rim of radius 28 from −9 to 9,
//! a bore of radius 6, and a V groove, the triangle (22, 0), (29, −5), (29, 5) turned about z.
use anvil_brep::{Frame, Solid};

use crate::shared::{cut_all, cylinder, polygon, turned, union};

pub fn build() -> Result<Solid, String> {
    let w = Frame::world();
    let body = union(&cylinder(&w, 12., -14., 14.)?, &cylinder(&w, 28., -9., 9.)?)?;
    // the groove's triangle reaches 1 past the rim's face
    let groove = turned(&w, polygon(&[(22., 0.), (29., -5.), (29., 5.)]));
    cut_all(body, [cylinder(&w, 6., -15., 15.), groove])
}
