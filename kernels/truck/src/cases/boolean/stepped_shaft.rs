//! "A stepped shaft on the z axis, made one solid. The first cylinder has radius 5 and runs from
//! z = 0 to z = 10. The second has radius 3 and runs from z = 10 to z = 20, standing on top of the
//! first."
use crate::{shapes, Built, Refused};
use truck_modeling::*;

const LOWER_RADIUS: f64 = 5.0;
const UPPER_RADIUS: f64 = 3.0;
const STEP: f64 = 10.0; // the length of each, and the z of the step

pub fn build() -> Built {
    let lower = shapes::cylinder(Point3::origin(), Vector3::unit_z(), LOWER_RADIUS, STEP);
    let step = Point3::new(0.0, 0.0, STEP);
    let upper = shapes::cylinder(step, Vector3::unit_z(), UPPER_RADIUS, STEP);
    let tolerance = shapes::boolean_tolerance(&lower, &upper);
    let shaft = truck_shapeops::or(&lower, &upper, tolerance)
        .ok_or_else(|| Refused::none_from("truck-shapeops `or`"))?;
    Ok(shaft.into())
}
