//! The half above z = 0 of the ring torus about the z axis whose tube has radius 2 about the circle
//! of radius 6, its flat the ring 4 ≤ r ≤ 8 at z = 0, its outer rim rounded to radius 1/2 by a
//! piece of the torus about the circle of radius 6 + √2 at z = 1/2; the round then deleted, the
//! flat running on outward and the torus down until they meet in the circle of radius 8 at z = 0.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_on, named, rounded_edge};
use cad_kernel::build::{kept, torus};
use cad_kernel::geom::Surface;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let b = named(kept(&torus([0.; 3], [0., 0., 1.], 6., 2.)?, [0.; 3], [0., 0., 1.], 0.)?);
    let b = rounded_edge(&b, [0., 8., 0.], 0.5)?;
    // the round: the face neither the flat nor the torus of tube 2
    let round = faces_on(&b, |s| !matches!(s, Surface::Plane(..)) && !matches!(s, Surface::Torus(_, _, r) if *r > 1.));
    deleting(&b, &round)
}
