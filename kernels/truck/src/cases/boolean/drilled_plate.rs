//! "A plate from the origin to (100, 20, 10). Drill nine holes of radius 2 through it along z, at
//! y = 10 and x = 10, 20, 30, … 90. Drill them one after another, each hole its own cut."
use crate::{shapes, Built, Refused};
use truck_modeling::*;

const LENGTH: f64 = 100.0; // in x
const WIDTH: f64 = 20.0; // in y
const THICKNESS: f64 = 10.0; // in z
const HOLE_RADIUS: f64 = 2.0;
const HOLE_Y: f64 = 10.0;
const HOLE_PITCH: f64 = 10.0; // the holes at x = 10, 20, … 90
const HOLES: usize = 9;
/// How far each drill runs past the plate's faces, so no face of the drill lies in one of the
/// plate's.
const OVERRUN: f64 = 1.0;

pub fn build() -> Built {
    let corners = [Point3::origin(), Point3::new(LENGTH, WIDTH, THICKNESS)];
    let mut plate: Solid = primitive::cuboid(BoundingBox::from_iter(corners));
    for k in 1..=HOLES {
        let x = HOLE_PITCH * k as f64;
        let centre = Point3::new(x, HOLE_Y, -OVERRUN);
        let height = THICKNESS + 2.0 * OVERRUN;
        let mut drill = shapes::cylinder(centre, Vector3::unit_z(), HOLE_RADIUS, height);
        // truck-shapeops cuts by intersecting with the tool turned inside out
        drill.not();
        let tolerance = shapes::boolean_tolerance(&plate, &drill);
        plate = truck_shapeops::and(&plate, &drill, tolerance).ok_or_else(|| {
            Refused::none_from(&format!("truck-shapeops `and` (the hole at x = {x})"))
        })?;
    }
    Ok(plate.into())
}
