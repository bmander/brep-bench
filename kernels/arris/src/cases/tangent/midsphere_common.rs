//! The solid common to a cube of side 10 centred on the origin and the ball about the origin of
//! radius 5√2, which touches each of the cube's twelve edges at its midpoint.
use crate::solids::ball;
use arris::math::{Control, Point3};
use arris::ops::{common, primitive_box, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::SQRT_2;

const HALF_SIDE: f64 = 5.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (cube, _) = primitive_box(
        m,
        Point3::new(-HALF_SIDE, -HALF_SIDE, -HALF_SIDE),
        Point3::new(HALF_SIDE, HALF_SIDE, HALF_SIDE),
        &Control::NONE,
    )?;
    // the distance from the centre to the middle of an edge
    let (sphere, _) = ball(m, Point3::origin(), HALF_SIDE * SQRT_2, &Control::NONE)?;
    let (trimmed, _) = common(m, cube, sphere, &Control::NONE)?;
    Ok(trimmed)
}
