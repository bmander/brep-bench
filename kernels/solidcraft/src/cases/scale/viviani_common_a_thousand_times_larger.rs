//! A ball of radius 10000 centred on the origin, and a cylinder of radius 5000 on the line
//! x = 5000, y = 0 from z = -12000 to z = 12000, whose wall touches the sphere from inside at
//! (10000, 0, 0): the tangent Viviani cases a thousand times larger, placed as they are (the
//! ball's poles on the y axis and its seam through (-10000, 0, 0), the cylinder's seam on the z
//! axis), with the same calls as `shared::tangent`'s ball and cylinder.
//! The solid common to both.
use crate::bench::{common, rotated, translated};
use crate::Refusal;
use solidcore_ops::{make_cylinder, make_sphere, Body};

/// The ball, turned a half turn about z (the seam to -x), then a quarter turn about x (the poles
/// to y).
pub fn ball() -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 10000.0)?;
    let ball = rotated(ball, [0.0; 3], [0.0, 0.0, 1.0], 180.0)?;
    rotated(ball, [0.0; 3], [1.0, 0.0, 0.0], 90.0)
}

/// The cylinder, turned a half turn about its axis (the seam to its side away from
/// (10000, 0, 0)), then moved.
pub fn cylinder() -> Result<Body, Refusal> {
    let c = make_cylinder([0.0, 0.0, 0.0], 5000.0, 24000.0)?;
    let c = rotated(c, [0.0; 3], [0.0, 0.0, 1.0], 180.0)?;
    translated(c, [5000.0, 0.0, -12000.0])
}

pub fn build() -> Result<Body, Refusal> {
    common(ball()?, cylinder()?)
}
