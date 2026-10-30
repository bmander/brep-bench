//! A disc of radius 20 from z = −5 to 0, with six holes of radius 2 through it, the first at
//! (12, 0) and each of the others the first turned 60° further about z, each hole its own cut.
use arris::math::nalgebra::UnitQuaternion;
use arris::math::{Axis, Control, Isometry, Point3, Vec3};
use arris::ops::{cut, primitive_cylinder, transform, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::TAU;

const RADIUS: f64 = 20.0;
const THICKNESS: f64 = 5.0;
const HOLES: usize = 6;
const HOLE_CIRCLE: f64 = 12.0;
const HOLE_RADIUS: f64 = 2.0;
/// How far the drill runs past each face, so its ends lie clear of the disc's.
const OVERRUN: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let bottom = Point3::new(0.0, 0.0, -THICKNESS);
    let (mut disc, _) =
        primitive_cylinder(m, Axis::z_at(bottom), RADIUS, THICKNESS, &Control::NONE)?;
    let base = Point3::new(HOLE_CIRCLE, 0.0, -THICKNESS - OVERRUN);
    let height = THICKNESS + 2.0 * OVERRUN;
    let (first, _) = primitive_cylinder(m, Axis::z_at(base), HOLE_RADIUS, height, &Control::NONE)?;
    for k in 0..HOLES {
        // the first drill indexed k sixths of a turn about the z axis
        let index = UnitQuaternion::from_axis_angle(&Vec3::z_axis(), TAU * k as f64 / HOLES as f64);
        let (drill, _) = transform(m, first, &Isometry::from_rotation(index), &Control::NONE)?;
        (disc, _) = cut(m, disc, drill, &Control::NONE)?;
    }
    Ok(disc)
}
