//! "A ball of radius 20 centred on the origin, with a round hole of radius 6 bored right through
//! it parallel to the y axis. The hole's axis is the line x = 5, z = 0, so the hole runs from
//! x = −1 to x = 11 and stays inside the ball's outline: it enters and leaves through the sphere
//! and never breaks out of its side. If you cut the hole with a cylinder, make it run past the ball
//! at both ends."
use crate::{shapes, Built, Refused};
use truck_modeling::*;

const BALL_RADIUS: f64 = 20.0;
const HOLE_RADIUS: f64 = 6.0;
const HOLE_AXIS: (f64, f64) = (5.0, 0.0); // x, z

pub fn build() -> Built {
    let ball = shapes::ball(Point3::origin(), BALL_RADIUS);
    // the drill runs from y = -25 to y = 25, past the ball at both ends
    let reach = BALL_RADIUS + 5.0;
    let foot = Point3::new(HOLE_AXIS.0, -reach, HOLE_AXIS.1);
    let mut drill = shapes::cylinder(foot, Vector3::unit_y(), HOLE_RADIUS, 2.0 * reach);
    // truck-shapeops cuts by intersecting with the tool turned inside out
    drill.not();
    let tolerance = shapes::boolean_tolerance(&ball, &drill);
    let pierced = truck_shapeops::and(&ball, &drill, tolerance)
        .ok_or_else(|| Refused::none_from("truck-shapeops `and`"))?;
    Ok(pierced.into())
}
