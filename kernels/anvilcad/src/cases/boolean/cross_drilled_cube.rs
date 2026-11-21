//! A cube from the origin to (30, 30, 30) drilled through by fifteen holes of radius 2.5: three
//! along x, three along y (at z = 15) and nine along z, on the lines 7.5, 15 and 22.5.
use anvil_brep::Solid;

use crate::shared::{block, cut_all, rod};

pub fn build() -> Result<Solid, String> {
    let at = [7.5, 15., 22.5];
    let mut drills = Vec::new();
    // each drill runs 1 past the faces at both ends
    for a in at {
        drills.push(rod([-1., a, 15.], [31., a, 15.], 2.5));
        drills.push(rod([a, -1., 15.], [a, 31., 15.], 2.5));
        for b in at {
            drills.push(rod([a, b, -1.], [a, b, 31.], 2.5));
        }
    }
    cut_all(block([0., 0., 0.], [30., 30., 30.])?, drills)
}
