//! The frustum on the z axis from radius 6 at z = 0 to radius 3.5 at z = 5, its side the cone
//! 2r + z = 12, joined to the steeper frustum from radius 3.5 at z = 5 to radius 2 at z = 6, which
//! bevels its top rim; the bevel's cone then deleted, the top running on outward from radius 2 and
//! the side up from z = 5 until they meet in the circle of radius 3 at z = 6.
//!
//! The beveled frustum is the two frustums, `build::cone`s, joined by `boolean::combined`; the
//! bevel is picked by a point on it and deleted (`defeature::deleted`).
use crate::sketch::deleted_through;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::cone;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = cone([0., 0., 0.], [0., 0., 1.], 6., 3.5, 5.)?;
    let bevel = cone([0., 0., 5.], [0., 0., 1.], 3.5, 2., 1.)?;
    let beveled = combined(&body, &bevel, Op::Union, 0.)?;
    deleted_through(&beveled, &[[2.75, 0., 5.5]])
}
