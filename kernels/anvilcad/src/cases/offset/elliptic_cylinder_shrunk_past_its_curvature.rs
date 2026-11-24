//! An elliptic cylinder of semi-axes 5 and 3, 10 high, shrunk by 2.5, past its least radius of curvature.
use anvil_brep::Solid;

use crate::shared::{elliptic_cylinder, offset};

pub fn build() -> Result<Solid, String> {
    offset(&elliptic_cylinder(5., 3., 0., 10.)?, -2.5)
}
