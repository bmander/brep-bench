//! The spherical sector of the ball of radius 5 about the origin inside the cone with its apex at
//! the origin about the z axis, cos α = 3/5, its rim (the circle of radius 4 at z = 3) chamfered
//! with two distances, 2 down the cone and 1 on the ball, each a straight line from the edge in the
//! plane square to it: the sector cut by the cone from the circle 2 down the side from the rim to
//! the circle on the ball 1 in a straight line from it.
//!
//! The sector is cadrs's sphere primitive in common with its cone primitive (apex at the origin,
//! radius 8 at z = 6); the rim goes into one Chamfer feature of two distances measured along the
//! faces' tangent planes, each given as the tangent distance that lands on the chord, the first on
//! the edge's first face.
use crate::bench::{ball, common, cone, p3, v3};
use crate::shared::chamfer::chord_two_distances_on_ball;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

const DOWN_CONE: f64 = 2.0;
const ON_BALL: f64 = 1.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = ball(k, p3(0.0, 0.0, 0.0), 5.0)?;
    let c = cone(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 0.0, 8.0, 6.0)?;
    let sector = common(k, b, c)?;
    // The rim through (0, −4, 3), ON_BALL on the ball about the origin, through its top.
    chord_two_distances_on_ball(
        k,
        sector,
        p3(0.0, -4.0, 3.0),
        p3(0.0, 0.0, 0.0),
        p3(0.0, 0.0, 5.0),
        ON_BALL,
        DOWN_CONE,
        None,
    )
}
