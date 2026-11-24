//! A cylinder of radius 6 on the z axis from z = 0 to z = 10, both rims rounded to radius 2 by
//! quarters of the tori whose tubes have radius 2 about the circles of radius 4 at z = 8 and at
//! z = 2; both rounds then deleted together, the top and the bottom running on outward from radius
//! 4 and the side up from z = 8 and down from z = 2 until they meet in the circles of radius 6 at
//! z = 10 and at z = 0.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{cylinder, deleting, faces_on, named, rounded_edge};
use cad_kernel::geom::Surface;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let b = named(cylinder([0., 0., 0.], 6., 10.)?);
    let b = rounded_edge(&b, [0., 6., 10.], 2.)?;
    let b = rounded_edge(&b, [0., 6., 0.], 2.)?;
    let rounds = faces_on(&b, |s| matches!(s, Surface::Torus(..)));
    deleting(&b, &rounds)
}
