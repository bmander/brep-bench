//! A ball of radius 5 on the origin united with a cylinder of the same radius on the z axis from
//! z = −3 to 3, the ball touching the wall from inside along its equator.
use crate::solids::ball;
use arris::math::{Axis, Control, Point3};
use arris::ops::{fuse, primitive_cylinder, OpError};
use arris::topo::{Body, Model};

const RADIUS: f64 = 5.0;
const HALF_HEIGHT: f64 = 3.0;

// workaround: no sphere primitive
pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (sphere, _) = ball(m, Point3::origin(), RADIUS, &Control::NONE)?;
    let base = Point3::new(0.0, 0.0, -HALF_HEIGHT);
    let (band, _) =
        primitive_cylinder(m, Axis::z_at(base), RADIUS, 2.0 * HALF_HEIGHT, &Control::NONE)?;
    let (joined, _) = fuse(m, sphere, band, &Control::NONE)?;
    Ok(joined)
}
