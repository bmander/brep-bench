//! The L through (0, 0), (20, 0), (20, 20), (15, 20), (15, 5), (0, 5) extruded 20 up z, shelled inward 1 thick, open at its face y = 5, its corners sharp.
use anvil_brep::Solid;

use crate::shared::{prism, shell_open};

pub fn build() -> Result<Vec<Solid>, String> {
    let l = prism(&[(0., 0.), (20., 0.), (20., 20.), (15., 20.), (15., 5.), (0., 5.)], 0., 20.)?;
    shell_open(&l, &[[7., 5., 10.]], 1.)
}
