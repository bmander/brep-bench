//! The half of the ball of radius 5 about the origin above z = 0, its rim rounded to radius 1 by a
//! piece of the torus whose tube has radius 1 about the circle of radius √15 at z = 1; the round
//! then deleted, the flat running on outward and the ball down until they meet in the circle of
//! radius 5 at z = 0.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_on, named, rounded_edge};
use cad_kernel::build::{kept, sphere};
use cad_kernel::geom::Surface;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let b = named(kept(&sphere([0.; 3], [0., 0., 1.], 5.)?, [0.; 3], [0., 0., 1.], 0.)?);
    let b = rounded_edge(&b, [0., 5., 0.], 1.)?;
    // the round: the face neither the ball nor the flat
    let round = faces_on(&b, |s| !matches!(s, Surface::Plane(..) | Surface::Sphere(..)));
    deleting(&b, &round)
}
