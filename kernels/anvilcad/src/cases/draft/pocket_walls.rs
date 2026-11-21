//! The block from the origin to (40, 30, 15) with a pocket from (8, 8, 5) to (32, 22, 15) cut into
//! its top, the pocket's walls drafted 5° about its floor, pulling along +z: it widens upward.
use anvil_brep::Solid;

use crate::shared::{block, draft, subtract};

pub fn build() -> Result<Solid, String> {
    let pocketed = subtract(&block([0., 0., 0.], [40., 30., 15.])?, &block([8., 8., 5.], [32., 22., 16.])?)?;
    let walls = [[20., 8., 10.], [32., 15., 10.], [20., 22., 10.], [8., 15., 10.]];
    draft(&pocketed, &walls, [0., 0., 5.], [0., 0., 1.], 5.)
}
