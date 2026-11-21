//! The common part of two tori about the z axis (middle circle 5, tube 2), their middle circles
//! in z = −1 and z = 1.
use anvil_brep::Solid;

use crate::shared::{at, intersect, torus};

pub fn build() -> Result<Solid, String> {
    intersect(&torus(&at([0., 0., -1.]), 5., 2.)?, &torus(&at([0., 0., 1.]), 5., 2.)?)
}
