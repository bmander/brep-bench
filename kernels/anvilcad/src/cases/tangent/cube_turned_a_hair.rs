//! A cube from (−5, −5, 0) to (5, 5, 10) united with a copy of itself turned 0.001° about the z
//! axis.
use anvil_brep::Solid;

use crate::shared::{block, turn_z, union};

pub fn build() -> Result<Solid, String> {
    let cube = block([-5., -5., 0.], [5., 5., 10.])?;
    union(&cube, &cube.transformed(&turn_z(0.001)))
}
