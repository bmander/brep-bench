//! The common part of two cylinders of radius 5 and length 30 crossing at right angles, one along
//! the x axis, the other parallel to y on the line x = 0, z = 9.99: they overlap 0.01 deep.
use anvil_brep::Solid;

use crate::shared::{intersect, rod};

pub fn build() -> Result<Solid, String> {
    intersect(&rod([-15., 0., 0.], [15., 0., 0.], 5.)?, &rod([0., -15., 9.99], [0., 15., 9.99], 5.)?)
}
