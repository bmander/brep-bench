//! Two cylinders of radius 5 on the z axis, from z = 0 to 10 and from 6 to 16, united: their walls
//! coincide between z = 6 and 10.
use arris::math::nalgebra::UnitQuaternion;
use arris::math::{Axis, Control, Isometry, Point3, Vec3};
use arris::ops::{fuse, primitive_cylinder, transform, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::FRAC_PI_2;

const RADIUS: f64 = 5.0;
const HEIGHT: f64 = 10.0;
const SECOND_BASE: f64 = 6.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (first, _) =
        primitive_cylinder(m, Axis::z_at(Point3::origin()), RADIUS, HEIGHT, &Control::NONE)?;
    let base = Point3::new(0.0, 0.0, SECOND_BASE);
    let (second, _) = primitive_cylinder(m, Axis::z_at(base), RADIUS, HEIGHT, &Control::NONE)?;
    // arris's cylinder has its seam where the axis frame's x points: the second turned a quarter
    // turn about z, so the coincident walls are not split in the same place
    let quarter = UnitQuaternion::from_axis_angle(&Vec3::z_axis(), FRAC_PI_2);
    let (second, _) = transform(m, second, &Isometry::from_rotation(quarter), &Control::NONE)?;
    let (joined, _) = fuse(m, first, second, &Control::NONE)?;
    Ok(joined)
}
