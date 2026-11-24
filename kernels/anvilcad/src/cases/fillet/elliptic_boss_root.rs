//! A plate 40 × 30 × 4 with an elliptic boss of semi-axes 10 and 6 on it, 6 high, its root
//! rounded to radius 2.
use anvil_brep::Solid;

use crate::shared::{block, elliptic_cylinder, fillet, union};

pub fn build() -> Result<Solid, String> {
    let plate = block([-20., -15., 0.], [20., 15., 4.])?;
    let boss = elliptic_cylinder(10., 6., 4., 10.)?;
    fillet(&union(&plate, &boss)?, &[[10., 0., 4.]], 2.)
}
