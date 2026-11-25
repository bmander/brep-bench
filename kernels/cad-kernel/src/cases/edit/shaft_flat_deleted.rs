//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a flat milled
//! along its whole length, everything with y < −4 cut away; the flat, the strip of the plane y = −4
//! from x = −3 to x = 3, then deleted, the cylinder closing over it and the ends growing back to
//! whole discs.
//!
//! The milled shaft is a `prism` of a `disc` cut by a box, a `prism` of a rectangle, standing on y
//! = −4 and running past both ends; the flat is picked by a point on it and deleted
//! (`defeature::deleted`).
use crate::sketch::{block, deleted_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let shaft = prism(&disc([0., 0., 0.], [0., 0., 1.], 5.), 0., 20.)?;
    let milled = combined(&shaft, &block([-10., -10., -1.], [10., -4., 21.])?, Op::Cut, 0.)?;
    deleted_through(&milled, &[[0., -4., 10.]])
}
