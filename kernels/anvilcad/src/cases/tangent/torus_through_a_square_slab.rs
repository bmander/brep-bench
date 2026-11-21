//! A torus about the z axis (middle circle 10, tube 3) united with the slab from (−13, −13, −1) to
//! (13, 13, 1), whose sides it touches at their middles.
use anvil_brep::Solid;

use crate::shared::{block, frame, torus, union};

pub fn build() -> Result<Solid, String> {
    // the torus turned 45° about z, so that its seam misses the points of contact
    let turned = frame([0., 0., 0.], [0., 0., 1.], [1., 1., 0.])?;
    union(&torus(&turned, 10., 3.)?, &block([-13., -13., -1.], [13., 13., 1.])?)
}
