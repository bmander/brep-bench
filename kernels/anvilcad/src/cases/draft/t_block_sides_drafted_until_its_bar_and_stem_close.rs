//! The T through (0, 0), (20, 0), (20, 4), (13, 4), (13, 16), (7, 16), (7, 4), (0, 4) extruded
//! from z = 0 to 8, its eight sides drafted 45° about the floor, pulling along +z: the bar and the
//! stem close in ridges below the top.
use anvil_brep::Solid;

use crate::shared::{draft, prism};

pub fn build() -> Result<Solid, String> {
    let t = prism(&[(0., 0.), (20., 0.), (20., 4.), (13., 4.), (13., 16.), (7., 16.), (7., 4.), (0., 4.)], 0., 8.)?;
    let sides = [
        [10., 0., 0.5], [20., 2., 0.5], [16., 4., 0.5], [13., 10., 0.5],
        [10., 16., 0.5], [7., 10., 0.5], [3., 4., 0.5], [0., 2., 0.5],
    ];
    draft(&t, &sides, [0., 0., 0.], [0., 0., 1.], 45.)
}
