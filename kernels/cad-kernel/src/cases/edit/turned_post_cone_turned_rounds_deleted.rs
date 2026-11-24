//! A turned post, the cylinder of radius 5 on the z axis from z = 0 to z = 6 and on it the cone
//! frustum from radius 5 at z = 6 to radius 2 at z = 10, the cone made turned 1 radian about the
//! axis before the two are joined, so the shoulder is two edges and its round of radius 1 two
//! pieces of a torus; both pieces then deleted, the cylinder running on up and the cone down until
//! they meet again in the circle of radius 5 at z = 6.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_on, named, rounded_edge};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{cone, disc, prism};
use cad_kernel::geom::{Rigid, Surface};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let post = prism(&disc([0., 0., 0.], [0., 0., 1.], 5.), 0., 6.)?;
    // the cone turned about the axis, so its seam stands apart from the cylinder's
    let top = cone([0., 0., 6.], [0., 0., 1.], 5., 2., 4.)?.moved(&Rigid::turn([0.; 3], [0., 0., 1.], 1.));
    let b = named(combined(&post, &top, Op::Union, 0.)?);
    let b = rounded_edge(&b, [0., 5., 6.], 1.)?;
    let round = faces_on(&b, &|s| matches!(s, Surface::Torus(..)));
    deleting(&b, &round)
}
