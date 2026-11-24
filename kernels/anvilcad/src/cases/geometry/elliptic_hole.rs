//! A block 24 × 16 × 8 with an elliptic hole of semi-axes 6 and 3 right through it.
use anvil_brep::Solid;

use crate::shared::{block_with_elliptic_hole};

pub fn build() -> Result<Solid, String> {
    block_with_elliptic_hole([-12., -8., 0.], [12., 8., 8.], 6., 3.)
}
