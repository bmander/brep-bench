//! Two cone frustums on the z axis made one solid, from radius 3 at z = 0 to radius 5 at z = 4 and
//! from there to radius 2 at z = 8, the upper made turned 2 radians about the axis before the two
//! are joined, so their ridge is two edges and its round of radius 1 two pieces of a torus; both
//! pieces then deleted, the lower cone running on up and the upper one down until they meet again
//! in the circle of radius 5 at z = 4.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_on, named, rounded_edge};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::cone;
use cad_kernel::geom::{Rigid, Surface};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let lower = cone([0., 0., 0.], [0., 0., 1.], 3., 5., 4.)?;
    // the upper frustum turned about the axis, so its seam stands apart from the lower one's
    let upper = cone([0., 0., 4.], [0., 0., 1.], 5., 2., 4.)?.moved(&Rigid::turn([0.; 3], [0., 0., 1.], 1.));
    let b = named(combined(&lower, &upper, Op::Union, 0.)?);
    let b = rounded_edge(&b, [0., 5., 4.], 1.)?;
    let round = faces_on(&b, &|s| matches!(s, Surface::Torus(..)));
    deleting(&b, &round)
}
