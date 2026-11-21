//! The L through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15) extruded 8 up z, shelled inward 1 thick, open at its top, the moved faces meeting in sharp edges.
use anvil_brep::Solid;

use crate::shared::{prism, shell_open};

pub fn build() -> Result<Vec<Solid>, String> {
    let l = prism(&[(0., 0.), (20., 0.), (20., 5.), (5., 5.), (5., 15.), (0., 15.)], 0., 8.)?;
    shell_open(&l, &[[2., 2., 8.]], 1.)
}
