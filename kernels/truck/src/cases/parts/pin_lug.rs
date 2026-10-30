//! "A flat lug: a plate from (0, −5, 0) to (80, 5, 40), so it is 80 long in x, 40 tall in z and 10
//! thick in y. A round pin hole of radius 8 goes straight through its thickness, along y, its axis
//! the line x = 60, z = 20. If you cut the hole with a cylinder, make it run past both faces of the
//! plate."
use crate::{shapes, Built, Refused};
use truck_modeling::*;

const LENGTH: f64 = 80.0; // in x
const HALF_THICKNESS: f64 = 5.0; // in y, either side of y = 0
const HEIGHT: f64 = 40.0; // in z
const PIN_RADIUS: f64 = 8.0;
const PIN_AXIS: (f64, f64) = (60.0, 20.0); // x, z
/// How far the drill runs past each face of the plate.
const OVERRUN: f64 = 1.0;

pub fn build() -> Built {
    let corners = [
        Point3::new(0.0, -HALF_THICKNESS, 0.0),
        Point3::new(LENGTH, HALF_THICKNESS, HEIGHT),
    ];
    let plate: Solid = primitive::cuboid(BoundingBox::from_iter(corners));
    let reach = HALF_THICKNESS + OVERRUN;
    let foot = Point3::new(PIN_AXIS.0, -reach, PIN_AXIS.1);
    let mut drill = shapes::cylinder(foot, Vector3::unit_y(), PIN_RADIUS, 2.0 * reach);
    // truck-shapeops cuts by intersecting with the tool turned inside out
    drill.not();
    let tolerance = shapes::boolean_tolerance(&plate, &drill);
    let lug = truck_shapeops::and(&plate, &drill, tolerance)
        .ok_or_else(|| Refused::none_from("truck-shapeops `and`"))?;
    Ok(lug.into())
}
