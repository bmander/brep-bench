//! The L through (0, 0), (30, 0), (30, 10), (10, 10), (10, 25), (0, 25) extruded from z = 0 to 12,
//! its six sides drafted 6° about the floor, pulling along +z.
use anvil_brep::Solid;

use crate::shared::{draft, prism};

pub fn build() -> Result<Solid, String> {
    let l = prism(&[(0., 0.), (30., 0.), (30., 10.), (10., 10.), (10., 25.), (0., 25.)], 0., 12.)?;
    let sides = [[15., 0., 2.], [30., 5., 2.], [20., 10., 2.], [10., 17., 2.], [5., 25., 2.], [0., 12., 2.]];
    draft(&l, &sides, [0., 0., 0.], [0., 0., 1.], 6.)
}
