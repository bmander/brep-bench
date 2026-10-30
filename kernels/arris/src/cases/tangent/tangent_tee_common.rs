//! The solid common to a cylinder of radius 5 along the x axis from x = −15 to 15 and one of
//! radius 3 parallel to y from y = −15 to 15 whose axis is the line x = 0, z = 2, the two tops
//! tangent at (0, 0, 5).
use arris::math::{Axis, Control, Point3, Vec3};
use arris::ops::{common, primitive_cylinder, OpError};
use arris::topo::{Body, Model};

const FIRST_RADIUS: f64 = 5.0;
const SECOND_RADIUS: f64 = 3.0;
const SECOND_Z: f64 = 2.0;
const LENGTH: f64 = 30.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let along_x = Axis::new(Point3::new(-LENGTH / 2.0, 0.0, 0.0), Vec3::x())?;
    let along_y = Axis::new(Point3::new(0.0, -LENGTH / 2.0, SECOND_Z), Vec3::y())?;
    let (first, _) = primitive_cylinder(m, along_x, FIRST_RADIUS, LENGTH, &Control::NONE)?;
    let (second, _) = primitive_cylinder(m, along_y, SECOND_RADIUS, LENGTH, &Control::NONE)?;
    let (shared, _) = common(m, first, second, &Control::NONE)?;
    Ok(shared)
}
