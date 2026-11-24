//! Two cone frustums on the z axis made one solid, from radius 3 at z = 0 to radius 5 at z = 4 and
//! from there to radius 2 at z = 8, their ridge, the circle of radius 5 at z = 4, rounded to radius
//! 1 by a piece of a torus tangent to both cones; the round then deleted, the lower cone running on
//! up and the upper one down until they meet again in the ridge.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_on, named, rounded_edge, turned};
use cad_kernel::geom::Surface;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let b = named(turned(&[[3., 0.], [5., 4.], [2., 8.]])?);
    let b = rounded_edge(&b, [0., 5., 4.], 1.)?;
    let round = faces_on(&b, &|s| matches!(s, Surface::Torus(..)));
    deleting(&b, &round)
}
