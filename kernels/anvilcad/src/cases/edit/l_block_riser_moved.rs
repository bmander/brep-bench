//! The L of section (0, 0), (40, 0), (40, 10), (15, 10), (15, 30), (0, 30) in xz from y = 0 to
//! 20, its riser x = 15 moved out 10 along +x.
use anvil_brep::Solid;

use crate::shared::{move_faces, prism_xz};

pub fn build() -> Result<Solid, String> {
    let l = prism_xz(&[(0., 0.), (40., 0.), (40., 10.), (15., 10.), (15., 30.), (0., 30.)], 0., 20.)?;
    // the riser faces +x, out of the solid: moving it 10 along +x moves it 10 out
    move_faces(&l, &[([15., 10., 20.], 10.)])
}
