//! A post, the cylinder of radius 3 on the z axis from z = 0 to z = 10, joined to a ball of radius
//! 5 about (0, 0, 10), the concave circle where the post meets the ball (radius 3 at z = 6)
//! chamfered with two distances, 2 down the post and 1 on the ball, each a straight line from the
//! edge in the plane square to it: the groove between the post and the ball filled up to the cone
//! from the circle on the post at z = 4 to the circle on the ball 1 in a straight line from the edge.
//!
//! The post is a circle extruded and the ball cadrs's sphere primitive, united; the concave edge
//! goes into one Chamfer feature of two distances, the first on the edge's first face.
use crate::bench::{ball, cylinder, p3, union, v3};
use crate::shared::chamfer::two_distances_on_ball;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

const DOWN_POST: f64 = 2.0;
const ON_BALL: f64 = 1.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let post = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 3.0, 10.0)?;
    let b = ball(k, p3(0.0, 0.0, 10.0), 5.0)?;
    let joined = union(k, post, b)?;
    // The concave edge through (0, −3, 6), ON_BALL on the face through the ball's top.
    two_distances_on_ball(k, joined, p3(0.0, -3.0, 6.0), p3(0.0, 0.0, 10.0), p3(0.0, 0.0, 15.0), ON_BALL, DOWN_POST)
}
