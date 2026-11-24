//! A turned post, the cylinder of radius 5 on the z axis from z = 0 to z = 6 and on it the cone
//! frustum from radius 5 at z = 6 to radius 2 at z = 10, its shoulder, the circle of radius 5 at
//! z = 6, rounded to radius 1 by a piece of a torus tangent to the cylinder and to the cone; the
//! round then deleted, the cylinder running on up and the cone down until they meet again in the
//! shoulder.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_on, named, rounded_edge, turned};
use cad_kernel::geom::Surface;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let b = named(turned(&[[5., 0.], [5., 6.], [2., 10.]])?);
    let b = rounded_edge(&b, [0., 5., 6.], 1.)?;
    let round = faces_on(&b, |s| matches!(s, Surface::Torus(..)));
    deleting(&b, &round)
}
