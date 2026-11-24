//! A solid elliptic cylinder of semi-axes 5 and 3, 10 high.
use anvil_brep::Solid;

use crate::shared::{elliptic_cylinder};

pub fn build() -> Result<Solid, String> {
    elliptic_cylinder(5., 3., 0., 10.)
}
