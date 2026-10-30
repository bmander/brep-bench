//! "Take a cube of side 10 between the origin and (10, 10, 10). Drill a hole of radius 3 straight
//! through it along z, centred at x = 5, y = 5. If you cut the hole with a cylinder, make it run
//! past both faces: from z = −1 to z = 11."
use crate::{shapes, Built, Refused};
use truck_modeling::*;

const SIDE: f64 = 10.0;
const HOLE_RADIUS: f64 = 3.0;
const HOLE_FROM: f64 = -1.0; // z
const HOLE_TO: f64 = 11.0;

pub fn build() -> Built {
    let corners = [Point3::origin(), Point3::new(SIDE, SIDE, SIDE)];
    let cube: Solid = primitive::cuboid(BoundingBox::from_iter(corners));
    let centre = Point3::new(SIDE / 2.0, SIDE / 2.0, HOLE_FROM);
    let mut drill = shapes::cylinder(centre, Vector3::unit_z(), HOLE_RADIUS, HOLE_TO - HOLE_FROM);
    // truck-shapeops cuts by intersecting with the tool turned inside out
    drill.not();
    let tolerance = shapes::boolean_tolerance(&cube, &drill);
    let drilled = truck_shapeops::and(&cube, &drill, tolerance)
        .ok_or_else(|| Refused::none_from("truck-shapeops `and`"))?;
    Ok(drilled.into())
}
