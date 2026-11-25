//! The box from the origin to (20, 10, 10), the two long edges of its top rounded to radius 2 by
//! the quarter cylinders about the lines y = 2, z = 8 and y = 8, z = 8; both rounds then deleted
//! together, the top running on from y = 2 and from y = 8 and the sides y = 0 and y = 10 up from
//! z = 8 until they meet in the edges at z = 10, and the ends x = 0 and x = 20 getting their
//! corners back.
//!
//! The rounded block is the box, a `prism`, each edge rolled by `fillet::roll` and its piece cut
//! away (`sketch::rolled`); the two quarter cylinders are picked by a point on each and deleted
//! together (`defeature::deleted`).
use crate::sketch::{block, deleted_through, rolled};
use cad_kernel::topo::Brep;
use std::f64::consts::SQRT_2;

pub fn build() -> Result<Brep, String> {
    let rounded = rolled(&block([0., 0., 0.], [20., 10., 10.])?, &[[10., 0., 10.], [10., 10., 10.]], 2.)?;
    // each round's middle, at 45° about its axis
    deleted_through(&rounded, &[[10., 2. - SQRT_2, 8. + SQRT_2], [10., 8. + SQRT_2, 8. + SQRT_2]])
}
