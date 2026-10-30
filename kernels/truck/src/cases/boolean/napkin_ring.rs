//! "A ball of radius 5 centred on the origin, with a hole of radius 3 bored right through it along
//! the z axis."
use crate::{shapes, Built, Refused};
use truck_modeling::*;

const BALL_RADIUS: f64 = 5.0;
const HOLE_RADIUS: f64 = 3.0;

pub fn build() -> Built {
    let ball = shapes::ball(Point3::origin(), BALL_RADIUS);
    // the drill runs from z = -10 to z = 10, well past both ends of the hole
    let foot = Point3::new(0.0, 0.0, -2.0 * BALL_RADIUS);
    let mut drill = shapes::cylinder(foot, Vector3::unit_z(), HOLE_RADIUS, 4.0 * BALL_RADIUS);
    // truck-shapeops cuts by intersecting with the tool turned inside out
    drill.not();
    let tolerance = shapes::boolean_tolerance(&ball, &drill);
    let ring = truck_shapeops::and(&ball, &drill, tolerance)
        .ok_or_else(|| Refused::none_from("truck-shapeops `and`"))?;
    Ok(ring.into())
}
