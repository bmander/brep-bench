//! A tee: a cylinder of radius 5 along the x axis from x = −15 to 15, united with one of radius 3
//! parallel to y from y = −15 to 15 whose axis is the line x = 0, z = 2, so that the two tops meet
//! tangentially at (0, 0, 5).
use arris::math::{Axis, Control, Point3, Vec3};
use arris::ops::{fuse, primitive_cylinder, OpError};
use arris::topo::{Body, Model};

const MAIN_RADIUS: f64 = 5.0;
const BRANCH_RADIUS: f64 = 3.0;
const BRANCH_Z: f64 = 2.0;
const LENGTH: f64 = 30.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let along_x = Axis::new(Point3::new(-LENGTH / 2.0, 0.0, 0.0), Vec3::x())?;
    let along_y = Axis::new(Point3::new(0.0, -LENGTH / 2.0, BRANCH_Z), Vec3::y())?;
    let (main, _) = primitive_cylinder(m, along_x, MAIN_RADIUS, LENGTH, &Control::NONE)?;
    let (branch, _) = primitive_cylinder(m, along_y, BRANCH_RADIUS, LENGTH, &Control::NONE)?;
    let (tee, _) = fuse(m, main, branch, &Control::NONE)?;
    Ok(tee)
}
