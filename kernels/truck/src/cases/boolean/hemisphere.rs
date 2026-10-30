//! "A ball of radius 5 centred on the origin, with everything below the xy plane cut away."
use crate::{shapes, Built, Refused};
use truck_modeling::*;

const RADIUS: f64 = 5.0;

// workaround: no sphere primitive
pub fn build() -> Built {
    let ball = shapes::ball(Point3::origin(), RADIUS);
    // everything below the xy plane, as far as the ball reaches: a block under it
    let reach = 2.0 * RADIUS;
    let mut below: Solid = primitive::cuboid(BoundingBox::from_iter([
        Point3::new(-reach, -reach, -reach),
        Point3::new(reach, reach, 0.0),
    ]));
    // truck-shapeops cuts by intersecting with the tool turned inside out
    below.not();
    let tolerance = shapes::boolean_tolerance(&ball, &below);
    let hemisphere = truck_shapeops::and(&ball, &below, tolerance)
        .ok_or_else(|| Refused::none_from("truck-shapeops `and`"))?;
    Ok(hemisphere.into())
}
