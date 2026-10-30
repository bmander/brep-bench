//! The solid common to two cylinders of radius 5 and length 30 crossing at right angles, one
//! along the x axis and one parallel to y on the line x = 0, z = 9.99: they overlap 0.01 deep.
use arris::math::{Axis, Control, Point3, Vec3};
use arris::ops::{common, primitive_cylinder, OpError};
use arris::topo::{Body, Model};

const RADIUS: f64 = 5.0;
const LENGTH: f64 = 30.0;
const OFFSET: f64 = 9.99;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let along_x = Axis::new(Point3::new(-LENGTH / 2.0, 0.0, 0.0), Vec3::x())?;
    let along_y = Axis::new(Point3::new(0.0, -LENGTH / 2.0, OFFSET), Vec3::y())?;
    let (first, _) = primitive_cylinder(m, along_x, RADIUS, LENGTH, &Control::NONE)?;
    let (second, _) = primitive_cylinder(m, along_y, RADIUS, LENGTH, &Control::NONE)?;
    let (shared, _) = common(m, first, second, &Control::NONE)?;
    Ok(shared)
}
