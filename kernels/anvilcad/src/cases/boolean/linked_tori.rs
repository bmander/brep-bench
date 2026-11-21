//! Two linked tori of middle circle 3 and tube 0.5, one about the z axis at the origin, the other
//! about a line parallel to y through (3, 0, 0); united, keeping both bodies.
use anvil_brep::Solid;
use anvil_brep::boolean::{BoolOp, boolean};

use crate::shared::{e, frame, torus};

pub fn build() -> Result<Vec<Solid>, String> {
    let a = torus(&frame([0., 0., 0.], [0., 0., 1.], [1., 0., 0.])?, 3., 0.5)?;
    let b = torus(&frame([3., 0., 0.], [0., 1., 0.], [1., 0., 0.])?, 3., 0.5)?;
    // the union of two solids that do not touch is both of them
    boolean(&a, &b, BoolOp::Union).map_err(e)
}
