//! A ball of radius 5 with a hole of radius 3 bored right through it along the z axis.
use crate::solids::ball;
use arris::math::{Axis, Control, Point3};
use arris::ops::{cut, primitive_cylinder, OpError};
use arris::topo::{Body, Model};

const RADIUS: f64 = 5.0;
const HOLE_RADIUS: f64 = 3.0;

// workaround: no sphere primitive
pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (sphere, _) = ball(m, Point3::origin(), RADIUS, &Control::NONE)?;
    // a bore twice the ball's diameter long, its ends well clear of the poles
    let base = Point3::new(0.0, 0.0, -2.0 * RADIUS);
    let (bore, _) =
        primitive_cylinder(m, Axis::z_at(base), HOLE_RADIUS, 4.0 * RADIUS, &Control::NONE)?;
    let (ring, _) = cut(m, sphere, bore, &Control::NONE)?;
    Ok(ring)
}
