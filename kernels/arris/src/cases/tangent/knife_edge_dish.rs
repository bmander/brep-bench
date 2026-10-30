//! A cylinder of radius 5 on the z axis from z = 0 to 10, less a ball of radius 5 about the middle
//! of its top face: the dish meets the wall tangentially in a knife edge.
use crate::solids::ball;
use arris::math::{Axis, Control, Point3};
use arris::ops::{cut, primitive_cylinder, OpError};
use arris::topo::{Body, Model};

const RADIUS: f64 = 5.0;
const HEIGHT: f64 = 10.0;

// workaround: no sphere primitive
pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (rod, _) =
        primitive_cylinder(m, Axis::z_at(Point3::origin()), RADIUS, HEIGHT, &Control::NONE)?;
    let (sphere, _) = ball(m, Point3::new(0.0, 0.0, HEIGHT), RADIUS, &Control::NONE)?;
    let (dished, _) = cut(m, rod, sphere, &Control::NONE)?;
    Ok(dished)
}
