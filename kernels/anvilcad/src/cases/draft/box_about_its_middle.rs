//! The box from the origin to (20, 12, 10), its four sides drafted 5° about the plane z = 5,
//! pulling along +z.
use anvil_brep::Solid;

use crate::shared::{block, draft};

pub fn build() -> Result<Solid, String> {
    let sides = [[10., 0., 5.], [20., 6., 5.], [10., 12., 5.], [0., 6., 5.]];
    draft(&block([0., 0., 0.], [20., 12., 10.])?, &sides, [0., 0., 5.], [0., 0., 1.], 5.)
}
