//! The box from the origin to (30, 10, 10), its four sides drafted 30° about the floor, pulling
//! along +z: the long sides meet below the top, which the draft consumes.
use anvil_brep::Solid;

use crate::shared::{block, draft};

pub fn build() -> Result<Solid, String> {
    let sides = [[15., 0., 2.], [30., 5., 2.], [15., 10., 2.], [0., 5., 2.]];
    draft(&block([0., 0., 0.], [30., 10., 10.])?, &sides, [0., 0., 0.], [0., 0., 1.], 30.)
}
