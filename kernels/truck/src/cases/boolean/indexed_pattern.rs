//! "A round disc of radius 20 on the z axis, from z = −5 to z = 0. Six round holes of radius 2 go
//! straight through it along z, their axes on the circle of radius 12 about the z axis: the first
//! at (12, 0), and each of the others the first turned about the z axis by 60°, 120°, 180°, 240°
//! and 300°. Cut them one after another, each hole its own cut. If you cut a hole with a cylinder,
//! make it run past both faces of the disc."
use crate::{shapes, Built, Refused};
use truck_modeling::*;

const DISC_RADIUS: f64 = 20.0;
const DISC_BOTTOM: f64 = -5.0; // z; the top is z = 0
const HOLE_RADIUS: f64 = 2.0;
const PITCH_RADIUS: f64 = 12.0;
const HOLES: usize = 6;
const INDEX: f64 = 60.0; // degrees between one hole and the next
/// How far each drill runs past the disc's faces.
const OVERRUN: f64 = 1.0;

pub fn build() -> Built {
    let thickness = -DISC_BOTTOM;
    let base = Point3::new(0.0, 0.0, DISC_BOTTOM);
    let mut disc = shapes::cylinder(base, Vector3::unit_z(), DISC_RADIUS, thickness);
    // the first drill, at (12, 0); the others are it turned about the z axis
    let foot = Point3::new(PITCH_RADIUS, 0.0, DISC_BOTTOM - OVERRUN);
    let first = shapes::cylinder(foot, Vector3::unit_z(), HOLE_RADIUS, thickness + 2.0 * OVERRUN);
    for k in 0..HOLES {
        let degrees = INDEX * k as f64;
        let turn = Rad(degrees.to_radians());
        let mut drill = builder::rotated(&first, Point3::origin(), Vector3::unit_z(), turn);
        // truck-shapeops cuts by intersecting with the tool turned inside out
        drill.not();
        let tolerance = shapes::boolean_tolerance(&disc, &drill);
        disc = truck_shapeops::and(&disc, &drill, tolerance).ok_or_else(|| {
            Refused::none_from(&format!("truck-shapeops `and` (the hole turned {degrees}°)"))
        })?;
    }
    Ok(disc.into())
}
