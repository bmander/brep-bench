//! The cone frustum on the z axis from radius 6 at z = 0 to radius 3 at z = 6, its top rim rounded
//! to radius 1 by a piece of the torus whose tube has radius 1 about the circle of radius
//! (7 − √5)/2 at z = 5; the round then deleted, the top running on outward and the cone up until
//! they meet in the circle of radius 3 at z = 6.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_on, named, rounded_edge};
use cad_kernel::build::cone;
use cad_kernel::geom::Surface;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let b = named(cone([0., 0., 0.], [0., 0., 1.], 6., 3., 6.)?);
    let b = rounded_edge(&b, [0., 3., 6.], 1.)?;
    let round = faces_on(&b, |s| matches!(s, Surface::Torus(..)));
    deleting(&b, &round)
}
