//! A post, the cylinder of radius 3 on the z axis from z = 0 to z = 10, joined to a ball of radius
//! 5 about (0, 0, 10), the concave circle where the post meets the ball (radius 3 at z = 6)
//! chamfered with two distances, 2 down the post and 1 on the ball, each a straight line from the
//! edge in the plane square to it: the groove between the post and the ball filled up to the cone
//! from the circle on the post at z = 4 to the circle on the ball 1 in a straight line from it.
use crate::solids::boolean::union;
use crate::solids::fillet::distances_ball_first;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{chamfer_edges_asymmetric, make_cylinder_brep, make_sphere_brep, BrepSolid};

const DOWN_POST: f64 = 2.0;
const ON_BALL: f64 = 1.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let post = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 3.0, 10.0)?;
    let ball = make_sphere_brep(p(0.0, 0.0, 10.0), 5.0, p(0.0, 0.0, 1.0))?;
    let joined = union(&post, &ball)?;
    // The concave edge, by a point on it. The kernel's two-distance chamfer sets back its first
    // distance on the edge's first face, each distance a chord in the plane square to the edge.
    let edge = p(0.0, -3.0, 6.0);
    let (d1, d2) = distances_ball_first(&joined, edge, p(0.0, 0.0, 10.0), ON_BALL, DOWN_POST)?;
    Ok(chamfer_edges_asymmetric(&joined, &[edge], None, d1, d2, None)?)
}
