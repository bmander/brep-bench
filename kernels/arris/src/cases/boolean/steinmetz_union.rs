//! The union of two cylinders of radius 5 and length 30 crossing along x and y.
use arris::math::{Axis, Control, Point3, Vec3};
use arris::ops::{fuse, primitive_cylinder, OpError};
use arris::topo::{Body, Model};

const RADIUS: f64 = 5.0;
const LENGTH: f64 = 30.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let along_x = Axis::new(Point3::new(-LENGTH / 2.0, 0.0, 0.0), Vec3::x())?;
    let along_y = Axis::new(Point3::new(0.0, -LENGTH / 2.0, 0.0), Vec3::y())?;
    let (first, _) = primitive_cylinder(m, along_x, RADIUS, LENGTH, &Control::NONE)?;
    let (second, _) = primitive_cylinder(m, along_y, RADIUS, LENGTH, &Control::NONE)?;
    let (joined, _) = fuse(m, first, second, &Control::NONE)?;
    Ok(joined)
}
