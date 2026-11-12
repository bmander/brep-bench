//! A ball of radius 5 about the origin bored through along the z axis by a hole of radius 2, the
//! upper circle where the bore meets the ball chamfered with two distances, 2 down the bore and 1
//! on the ball, each a straight line from the edge in the plane square to it: the bored ball cut
//! by the cone from the circle on the bore at z = √21 − 2 to the circle on the ball 1 in a straight
//! line from the rim.
//!
//! The ball is cadrs's sphere primitive less a cylinder running past both its sides; the upper rim
//! goes into one Chamfer feature of two distances measured along the faces' tangent planes, each
//! given as the tangent distance that lands on the chord, the first on the edge's first face.
use crate::bench::{ball, cut, cylinder, p3, v3};
use crate::shared::chamfer::chord_two_distances_on_ball;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

const DOWN_BORE: f64 = 2.0;
const ON_BALL: f64 = 1.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = ball(k, p3(0.0, 0.0, 0.0), 5.0)?;
    let bore = cylinder(k, p3(0.0, 0.0, -10.0), v3(0.0, 0.0, 1.0), 2.0, 20.0)?;
    let bored = cut(k, b, bore)?;
    // The upper rim through (0, −2, √21), ON_BALL on the face through (5, 0, 0).
    let rim = p3(0.0, -2.0, 21f64.sqrt());
    chord_two_distances_on_ball(
        k,
        bored,
        rim,
        p3(0.0, 0.0, 0.0),
        p3(5.0, 0.0, 0.0),
        ON_BALL,
        DOWN_BORE,
        None,
    )
}
