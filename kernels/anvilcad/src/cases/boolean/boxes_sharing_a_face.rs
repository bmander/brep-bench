//! Two cubes of side 10 side by side along x, touching along a whole face, joined into one.
use anvil_brep::Solid;

use crate::shared::{block, union};

pub fn build() -> Result<Solid, String> {
    let a = block([0., 0., 0.], [10., 10., 10.])?;
    let b = block([10., 0., 0.], [20., 10., 10.])?;
    union(&a, &b)
}
