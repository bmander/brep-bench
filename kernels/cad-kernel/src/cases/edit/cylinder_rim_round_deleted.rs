//! A cylinder of radius 6 on the z axis from z = 0 to z = 10, its top rim rounded to radius 2 by a
//! quarter of the torus whose tube has radius 2 about the circle of radius 4 at z = 8; the round
//! then deleted, the top running on from radius 4 and the side from z = 8 until they meet in the
//! circle of radius 6 at z = 10.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_on, named, rounded_edge};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::Surface;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let b = named(prism(&disc([0., 0., 0.], [0., 0., 1.], 6.), 0., 10.)?);
    let b = rounded_edge(&b, [0., 6., 10.], 2.)?;
    let round = faces_on(&b, &|s| matches!(s, Surface::Torus(..)));
    deleting(&b, &round)
}
