//! The box from (−10, −10, 0) to (10, 10, 10) with a blind bore of radius 4 about the z axis sunk
//! from its top to a floor at z = 4, the edge where the floor meets the wall beveled: the wall
//! stops at z = 5 and a cone runs from there down and in to the floor, the disk of radius 3; the
//! bevel then deleted, the floor running on outward from radius 3 and the wall down from z = 5
//! until they meet in the circle of radius 4 at z = 4.
//!
//! The beveled bore is the block, a `prism`, less the cutter: a `prism` of a `disc` from z = 5 up
//! past the top joined to the frustum `build::cone` from the floor up to it; the bevel is picked by
//! a point on it and deleted (`defeature::deleted`).
use crate::sketch::{block, deleted_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{cone, disc, prism};
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let bore = prism(&disc([0., 0., 5.], UP, 4.), 0., 6.)?;
    let cutter = combined(&bore, &cone([0., 0., 4.], UP, 3., 4., 1.)?, Op::Union, 0.)?;
    let bored = combined(&block([-10., -10., 0.], [10., 10., 10.])?, &cutter, Op::Cut, 0.)?;
    deleted_through(&bored, &[[3.5, 0., 4.5]])
}
