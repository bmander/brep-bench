//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with two opposite flats
//! milled along its whole length, everything with |x| > 4 cut away, the strips of the planes x = 4
//! and x = −4 from y = −3 to y = 3; both flats then deleted together, the cylinder closing over
//! both and the ends growing back to whole discs.
//!
//! The milled shaft is a `prism` of a `disc` cut by two boxes, `prism`s of rectangles, standing on
//! x = 4 and x = −4 and running past both ends; the flats are picked by a point on each and deleted
//! together (`defeature::deleted`).
use crate::sketch::{block, deleted_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let shaft = prism(&disc([0., 0., 0.], [0., 0., 1.], 5.), 0., 20.)?;
    let once = combined(&shaft, &block([4., -10., -1.], [10., 10., 21.])?, Op::Cut, 0.)?;
    let milled = combined(&once, &block([-10., -10., -1.], [-4., 10., 21.])?, Op::Cut, 0.)?;
    deleted_through(&milled, &[[4., 0., 10.], [-4., 0., 10.]])
}
