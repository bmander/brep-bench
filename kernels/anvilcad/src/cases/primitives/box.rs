//! A rectangular block 10 long in x, 20 in y and 30 in z, from the origin to (10, 20, 30).
use anvil_brep::Solid;

use crate::shared::block;

pub fn build() -> Result<Solid, String> {
    // the 10 × 20 rectangle in the xy plane, extruded from z = 0 to z = 30
    block([0., 0., 0.], [10., 20., 30.])
}
