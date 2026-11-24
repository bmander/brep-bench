//! A plate 30 × 20 × 4 with an elliptic hole of semi-axes 6 and 3, the hole's top rim chamfered by 1.
use anvil_brep::Solid;

use crate::shared::{block_with_elliptic_hole, chamfer};

pub fn build() -> Result<Solid, String> {
    chamfer(&block_with_elliptic_hole([-15., -10., 0.], [15., 10., 4.], 6., 3.)?, &[[6., 0., 4.]], 1.)
}
