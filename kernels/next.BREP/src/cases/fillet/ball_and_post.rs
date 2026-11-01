//! A ball of radius 10 about the origin joined to a post, the cylinder of radius 4 on the z axis
//! from z = 0 to z = 15, and the circle where the post leaves the ball (radius 4 at z = √84)
//! rounded to radius 2, adding material: a torus tangent to the ball and to the post.
use crate::solids::fillet::{on_circle, union};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_cylinder_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 10.0, p(0.0, 0.0, 1.0))?;
    let post = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 4.0, 15.0)?;
    let joined = union(&ball, &post)?;
    let root = on_circle(4.0, 84f64.sqrt(), 1.0);
    Ok(fillet_edges(&joined, &[root], None, 2.0, false, None)?)
}
