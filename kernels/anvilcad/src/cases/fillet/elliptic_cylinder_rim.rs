//! An elliptic cylinder of semi-axes 10 and 6, 8 high, its top rim rounded to radius 2.
use anvil_brep::Solid;

use crate::shared::{elliptic_cylinder, fillet};

pub fn build() -> Result<Solid, String> {
    fillet(&elliptic_cylinder(10., 6., 0., 8.)?, &[[10., 0., 8.]], 2.)
}
