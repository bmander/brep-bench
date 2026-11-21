//! The box from the origin to (20, 12, 10), its side x = 20 drafted 10° about the floor, pulling
//! along +z.
use anvil_brep::Solid;

use crate::shared::{block, draft};

pub fn build() -> Result<Solid, String> {
    draft(&block([0., 0., 0.], [20., 12., 10.])?, &[[20., 6., 5.]], [0., 0., 0.], [0., 0., 1.], 10.)
}
