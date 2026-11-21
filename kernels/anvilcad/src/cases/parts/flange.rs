//! A flange on the z axis: a plate of radius 32 from z = 0 to 6, a hub of radius 15 on it up to
//! z = 16, a bore of radius 8 through both, and six bolt holes of radius 3 through the plate on
//! the circle of radius 24, one every 60°.
use anvil_brep::{Frame, Solid};

use crate::shared::{at, cut_all, cylinder, turn_z, union};

pub fn build() -> Result<Solid, String> {
    let w = Frame::world();
    let body = union(&cylinder(&w, 32., 0., 6.)?, &cylinder(&w, 15., 6., 16.)?)?;
    let hole = cylinder(&at([24., 0., 0.]), 3., -1., 7.)?;
    let mut cuts = vec![cylinder(&w, 8., -1., 17.)];
    cuts.extend((0..6).map(|k| Ok(hole.transformed(&turn_z(60. * k as f64)))));
    cut_all(body, cuts)
}
