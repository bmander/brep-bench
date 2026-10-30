//! A stepped shaft on the z axis: radius 5 from z = 0 to 10, radius 3 from 10 to 20.
use arris::math::{Axis, Control, Point3};
use arris::ops::{fuse, primitive_cylinder, OpError};
use arris::topo::{Body, Model};

const STEP: f64 = 10.0;
const LOWER_RADIUS: f64 = 5.0;
const UPPER_RADIUS: f64 = 3.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (lower, _) =
        primitive_cylinder(m, Axis::z_at(Point3::origin()), LOWER_RADIUS, STEP, &Control::NONE)?;
    let shoulder = Point3::new(0.0, 0.0, STEP);
    let (upper, _) =
        primitive_cylinder(m, Axis::z_at(shoulder), UPPER_RADIUS, STEP, &Control::NONE)?;
    let (shaft, _) = fuse(m, lower, upper, &Control::NONE)?;
    Ok(shaft)
}
