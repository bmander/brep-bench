//! The cylinder of radius 6 on the z axis from z = 0 to z = 8 with the frustum from radius 6 at
//! z = 8 to radius 4 at z = 10 on it, a cylinder to z = 10 whose top rim is beveled 2 back on each
//! face; the bevel, the frustum's side, then deleted, the top running on from radius 4 and the side
//! from z = 8 until they meet in the circle of radius 6 at z = 10.
//!
//! The beveled cylinder is a `prism` of a `disc` joined to the frustum `build::cone`; the bevel is
//! picked by a point on it and deleted (`defeature::deleted`).
use crate::sketch::deleted_through;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{cone, disc, prism};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = prism(&disc([0., 0., 0.], [0., 0., 1.], 6.), 0., 8.)?;
    let bevel = cone([0., 0., 8.], [0., 0., 1.], 6., 4., 2.)?;
    let beveled = combined(&body, &bevel, Op::Union, 0.)?;
    deleted_through(&beveled, &[[5., 0., 9.]])
}
