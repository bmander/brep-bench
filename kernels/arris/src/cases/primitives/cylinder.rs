//! A solid cylinder of radius 5 and height 10 on the z axis, standing on the xy plane.
use arris::math::{Axis, Control, Point3};
use arris::ops::{primitive_cylinder, OpError};
use arris::topo::{Body, Model};

const RADIUS: f64 = 5.0;
const HEIGHT: f64 = 10.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (cylinder, _) =
        primitive_cylinder(m, Axis::z_at(Point3::origin()), RADIUS, HEIGHT, &Control::NONE)?;
    Ok(cylinder)
}
