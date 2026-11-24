//! The cone frustum on the z axis from radius 6 at z = 0 to radius 3 at z = 6, its side the cone
//! 2r + z = 12, both rims rounded to radius 3/2 by pieces of tori tangent to each end and to the
//! cone; the top round alone then deleted, the top running on outward and the cone up until they
//! meet in the circle of radius 3 at z = 6, the base's round staying.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_on, named, rounded_edge};
use cad_kernel::build::cone;
use cad_kernel::geom::Surface;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let b = named(cone([0., 0., 0.], [0., 0., 1.], 6., 3., 6.)?);
    let b = rounded_edge(&b, [0., 3., 6.], 1.5)?;
    let b = rounded_edge(&b, [0., 6., 0.], 1.5)?;
    // the top's round: the torus whose middle circle stands above z = 3
    let round = faces_on(&b, |s| matches!(s, Surface::Torus(f, ..) if f.o[2] > 3.));
    deleting(&b, &round)
}
