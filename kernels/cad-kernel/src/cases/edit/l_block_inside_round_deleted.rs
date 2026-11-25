//! The L-shaped block through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), 8 high, its
//! inside corner along x = 5, y = 5 rounded to radius 2 by the quarter cylinder about the line
//! x = 7, y = 7; the round then deleted, the face x = 5 running back from y = 7 and the face y = 5
//! from x = 7 until they meet in the edge x = 5, y = 5.
//!
//! The rounded L-block is `sketch::l_block`, its inside edge rolled by `fillet::roll` and the
//! round's piece joined into the corner (`sketch::rolled`); the quarter cylinder is picked by a
//! point on it and deleted (`defeature::deleted`).
use crate::sketch::{deleted_through, l_block, rolled};
use cad_kernel::topo::Brep;
use std::f64::consts::SQRT_2;

pub fn build() -> Result<Brep, String> {
    let rounded = rolled(&l_block()?, &[[5., 5., 4.]], 2.)?;
    // the round's middle, at 45° about its axis x = 7, y = 7, toward the corner
    deleted_through(&rounded, &[[7. - SQRT_2, 7. - SQRT_2, 4.]])
}
