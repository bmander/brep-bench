//! The L through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15) extruded from z = 0 to 8, its
//! six sides drafted 30° about the floor, pulling along +z: the arms close in ridges below the top.
use anvil_brep::Solid;

use crate::shared::{draft, prism};

pub fn build() -> Result<Solid, String> {
    let l = prism(&[(0., 0.), (20., 0.), (20., 5.), (5., 5.), (5., 15.), (0., 15.)], 0., 8.)?;
    let sides = [[10., 0., 1.], [20., 2.5, 1.], [12., 5., 1.], [5., 10., 1.], [2.5, 15., 1.], [0., 7., 1.]];
    draft(&l, &sides, [0., 0., 0.], [0., 0., 1.], 30.)
}
