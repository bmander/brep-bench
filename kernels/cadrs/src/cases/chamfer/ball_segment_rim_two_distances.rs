//! The part of a ball of radius 5 about the origin below the plane z = 3, the circle where its flat
//! face meets the ball (radius 4 at z = 3) chamfered with two distances, 2 on the flat face and 1
//! on the ball, each a straight line from the edge in the plane square to it: the cut ball cut
//! again by the cone from the circle of radius 2 on the flat face to the circle on the ball 1 in a
//! straight line from the rim.
//!
//! The ball is cadrs's sphere primitive, kept below z = 3 by its common with a block; the rim goes
//! into one Chamfer feature of two distances measured along the faces' tangent planes, each given
//! as the tangent distance that lands on the chord, the first on the edge's first face.
use crate::bench::{ball, block, common, p3};
use crate::shared::chamfer::chord_two_distances_on_ball;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

const ON_FLAT: f64 = 2.0;
const ON_BALL: f64 = 1.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = ball(k, p3(0.0, 0.0, 0.0), 5.0)?;
    let below = block(k, p3(-20.0, -20.0, -20.0), p3(20.0, 20.0, 3.0))?;
    let cut_ball = common(k, b, below)?;
    // The rim through (0, −4, 3), ON_BALL on the ball about the origin, through its bottom.
    chord_two_distances_on_ball(
        k,
        cut_ball,
        p3(0.0, -4.0, 3.0),
        p3(0.0, 0.0, 0.0),
        p3(0.0, 0.0, -5.0),
        ON_BALL,
        ON_FLAT,
        None,
    )
}
