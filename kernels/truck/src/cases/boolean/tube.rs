//! "A cylinder of radius 5 on the z axis, from z = 0 to z = 10, with a hole of radius 3 bored along
//! the axis. If you cut the hole with a cylinder, make it run past both ends: from z = −1 to
//! z = 11."
use crate::{shapes, Built, Refused};
use truck_modeling::*;

const OUTER_RADIUS: f64 = 5.0;
const HOLE_RADIUS: f64 = 3.0;
const LENGTH: f64 = 10.0;
const HOLE_FROM: f64 = -1.0; // z
const HOLE_TO: f64 = 11.0;

pub fn build() -> Built {
    let rod = shapes::cylinder(Point3::origin(), Vector3::unit_z(), OUTER_RADIUS, LENGTH);
    let foot = Point3::new(0.0, 0.0, HOLE_FROM);
    let mut drill = shapes::cylinder(foot, Vector3::unit_z(), HOLE_RADIUS, HOLE_TO - HOLE_FROM);
    // truck-shapeops cuts by intersecting with the tool turned inside out
    drill.not();
    let tolerance = shapes::boolean_tolerance(&rod, &drill);
    let tube = truck_shapeops::and(&rod, &drill, tolerance)
        .ok_or_else(|| Refused::none_from("truck-shapeops `and`"))?;
    Ok(tube.into())
}
