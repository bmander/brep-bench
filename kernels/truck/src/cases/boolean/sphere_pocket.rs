//! "A block from (−10, −10, −10) to (10, 10, 0). Cut from it a ball of radius 5 centred on the
//! origin, leaving a hemispherical pocket in its top face."
use crate::{shapes, Built, Refused};
use truck_modeling::*;

const BALL_RADIUS: f64 = 5.0;

pub fn build() -> Built {
    let block: Solid = primitive::cuboid(BoundingBox::from_iter([
        Point3::new(-10.0, -10.0, -10.0),
        Point3::new(10.0, 10.0, 0.0),
    ]));
    let mut ball = shapes::ball(Point3::origin(), BALL_RADIUS);
    // truck-shapeops cuts by intersecting with the tool turned inside out
    ball.not();
    let tolerance = shapes::boolean_tolerance(&block, &ball);
    let pocketed = truck_shapeops::and(&block, &ball, tolerance)
        .ok_or_else(|| Refused::none_from("truck-shapeops `and`"))?;
    Ok(pocketed.into())
}
