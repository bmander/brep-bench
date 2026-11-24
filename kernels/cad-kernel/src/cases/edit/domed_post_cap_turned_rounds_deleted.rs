//! A post, the cylinder of radius 4 on the z axis from z = 0 to z = 6, capped by the part above
//! z = 6 of the ball of radius 5 about (0, 0, 3), the cap made turned 1.5 radians about the axis
//! before the two are joined, so the rim at z = 6 is two edges and its round of radius 1 two pieces
//! of a torus; both pieces then deleted, the cylinder running on up and the dome down until they
//! meet again in the circle of radius 4 at z = 6.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_on, named, rounded_edge};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, kept, prism, sphere};
use cad_kernel::geom::{Rigid, Surface};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let post = prism(&disc([0., 0., 0.], [0., 0., 1.], 4.), 0., 6.)?;
    // the cap turned about the axis, so its seam stands apart from the cylinder's
    let dome = kept(&sphere([0., 0., 3.], [0., 0., 1.], 5.)?, [0., 0., 6.], [0., 0., 1.], 0.)?
        .moved(&Rigid::turn([0.; 3], [0., 0., 1.], 1.));
    let b = named(combined(&post, &dome, Op::Union, 0.)?);
    let b = rounded_edge(&b, [0., 4., 6.], 1.)?;
    // the round: the face neither the post's cylinder, its dome nor its foot
    let round = faces_on(&b, &|s| !matches!(s, Surface::Plane(..) | Surface::Cylinder(..) | Surface::Sphere(..)));
    deleting(&b, &round)
}
