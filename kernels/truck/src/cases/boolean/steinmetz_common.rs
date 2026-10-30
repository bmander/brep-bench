//! "Two cylinders of radius 5 and length 30 that cross at right angles. The first lies along the x
//! axis, from x = −15 to 15; the second along the y axis, from y = −15 to 15. Make the solid common
//! to both. Their surfaces meet in two ellipses, which cross each other at two points."
use crate::{shapes, Built, Refused};
use truck_modeling::*;

const RADIUS: f64 = 5.0;
const LENGTH: f64 = 30.0;

pub fn build() -> Built {
    let half = LENGTH / 2.0;
    let along_x = shapes::cylinder(Point3::new(-half, 0.0, 0.0), Vector3::unit_x(), RADIUS, LENGTH);
    let along_y = shapes::cylinder(Point3::new(0.0, -half, 0.0), Vector3::unit_y(), RADIUS, LENGTH);
    let tolerance = shapes::boolean_tolerance(&along_x, &along_y);
    let common = truck_shapeops::and(&along_x, &along_y, tolerance)
        .ok_or_else(|| Refused::none_from("truck-shapeops `and`"))?;
    Ok(common.into())
}
