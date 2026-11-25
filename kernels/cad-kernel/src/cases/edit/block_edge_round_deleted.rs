//! The box from the origin to (20, 10, 10), the edge where its top, z = 10, meets its side x = 20
//! rounded to radius 2 by the quarter cylinder about the line x = 18, z = 8; the round then
//! deleted, the top running on from x = 18 and the side from z = 8 until they meet in the edge
//! x = 20, z = 10, and the ends y = 0 and y = 10 getting their corners back.
//!
//! The rounded block is the box, a `prism`, its edge rolled by `fillet::roll` and the round's piece
//! cut away (`sketch::rolled`); the quarter cylinder is picked by a point on it and deleted
//! (`defeature::deleted`).
use crate::sketch::{block, deleted_through, rolled};
use cad_kernel::topo::Brep;
use std::f64::consts::SQRT_2;

pub fn build() -> Result<Brep, String> {
    let rounded = rolled(&block([0., 0., 0.], [20., 10., 10.])?, &[[20., 5., 10.]], 2.)?;
    // the round's middle, at 45° about its axis x = 18, z = 8
    deleted_through(&rounded, &[[18. + SQRT_2, 5., 8. + SQRT_2]])
}
