//! A ball of radius 20 at the origin with a hole of radius 6 bored through it along the line
//! x = 5, z = 0, parallel to y.
use crate::solids::ball;
use arris::math::{Axis, Control, Point3, Vec3};
use arris::ops::{cut, primitive_cylinder, OpError};
use arris::topo::{Body, Model};

const RADIUS: f64 = 20.0;
const HOLE_RADIUS: f64 = 6.0;
const HOLE_X: f64 = 5.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (sphere, _) = ball(m, Point3::origin(), RADIUS, &Control::NONE)?;
    // the drill runs past the ball at both ends
    let start = Point3::new(HOLE_X, -2.0 * RADIUS, 0.0);
    let (drill, _) = primitive_cylinder(m, Axis::new(start, Vec3::y())?, HOLE_RADIUS,
                                        4.0 * RADIUS, &Control::NONE)?;
    let (pierced, _) = cut(m, sphere, drill, &Control::NONE)?;
    Ok(pierced)
}
