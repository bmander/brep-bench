//! The block from (−15, −15, 0) to (15, 15, 20) bored through along z by a hole of radius 6, the
//! hole's wall drafted 3° about the floor, pulling along +z: the hole widens toward the top.
use anvil_brep::{Frame, Solid};

use crate::shared::{block, cylinder, draft, ring, subtract};

pub fn build() -> Result<Solid, String> {
    let bored = subtract(&block([-15., -15., 0.], [15., 15., 20.])?, &cylinder(&Frame::world(), 6., -1., 21.)?)?;
    // narrowing the body as it rises widens the hole
    draft(&bored, &[ring(6., 10.)], [0., 0., 0.], [0., 0., 1.], 3.)
}
