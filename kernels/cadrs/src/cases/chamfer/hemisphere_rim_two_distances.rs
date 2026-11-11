//! The upper half of a ball of radius 5 about the origin, the circle where its flat face meets the
//! ball chamfered with two distances, 2 on the flat face and 1 on the ball, each a straight line
//! from the edge in the plane square to it: the hemisphere cut by the cone from the circle of
//! radius 3 on the flat face to the circle on the ball 1 in a straight line from the rim.
//!
//! The ball is cadrs's sphere primitive, kept above z = 0 by its common with a block; the rim goes
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
    let upper = block(k, p3(-20.0, -20.0, 0.0), p3(20.0, 20.0, 20.0))?;
    let hemisphere = common(k, b, upper)?;
    // The rim through (0, −5, 0), ON_BALL on the ball about the origin, through its top.
    chord_two_distances_on_ball(
        k,
        hemisphere,
        p3(0.0, -5.0, 0.0),
        p3(0.0, 0.0, 0.0),
        p3(0.0, 0.0, 5.0),
        ON_BALL,
        ON_FLAT,
        None,
    )
}
