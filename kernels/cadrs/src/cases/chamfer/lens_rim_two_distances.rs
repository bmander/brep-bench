//! The lens common to two balls of radius 5 about (0, 0, 3) and (0, 0, −3), its rim (the circle of
//! radius 4 at z = 0) chamfered with two distances, 2 on its upper face (on the ball about
//! (0, 0, −3)) and 1 on its lower face, each a straight line from the edge in the plane square to
//! it: the lens cut by the cone between the two circles those chords reach.
//!
//! The balls are cadrs's sphere primitive, their common part taken; the rim goes into one Chamfer
//! feature of two distances measured along the faces' tangent planes, each given as the tangent
//! distance that lands on the chord, the first on the edge's first face.
use crate::bench::{ball, common, p3};
use crate::shared::chamfer::chord_two_distances_on_ball;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

const ON_UPPER: f64 = 2.0;
const ON_LOWER: f64 = 1.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let above = ball(k, p3(0.0, 0.0, 3.0), 5.0)?;
    let below = ball(k, p3(0.0, 0.0, -3.0), 5.0)?;
    let lens = common(k, above, below)?;
    // The rim through (0, −4, 0), ON_UPPER on the upper face, through (0, 0, 2).
    chord_two_distances_on_ball(
        k,
        lens,
        p3(0.0, -4.0, 0.0),
        p3(0.0, 0.0, -3.0),
        p3(0.0, 0.0, 2.0),
        ON_UPPER,
        ON_LOWER,
        Some(5.0),
    )
}
