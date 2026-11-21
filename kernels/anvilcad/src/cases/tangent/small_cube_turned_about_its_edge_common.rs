//! A cube of side 0.001 from the origin and a copy of it turned 1.2° about the z axis, the line of
//! one of its edges: their common part.
use anvil_brep::Solid;

use crate::shared::{block, intersect, turn_z};

pub fn build() -> Result<Solid, String> {
    let cube = block([0., 0., 0.], [0.001, 0.001, 0.001])?;
    intersect(&cube, &cube.transformed(&turn_z(1.2)))
}
