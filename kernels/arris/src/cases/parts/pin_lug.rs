//! A lug 80 long in x, 40 tall in z and 10 thick in y, with a pin hole of radius 8 through its
//! thickness on the line x = 60, z = 20.
use arris::math::{Axis, Control, Point3, Vec3};
use arris::ops::{cut, primitive_box, primitive_cylinder, OpError};
use arris::topo::{Body, Model};

const LENGTH: f64 = 80.0;
const THICKNESS: f64 = 10.0;
const HEIGHT: f64 = 40.0;
const PIN_X: f64 = 60.0;
const PIN_Z: f64 = 20.0;
const PIN_RADIUS: f64 = 8.0;
/// How far the drill runs past each face, so its ends lie clear of the plate's.
const OVERRUN: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (plate, _) = primitive_box(
        m,
        Point3::new(0.0, -THICKNESS / 2.0, 0.0),
        Point3::new(LENGTH, THICKNESS / 2.0, HEIGHT),
        &Control::NONE,
    )?;
    let start = Point3::new(PIN_X, -THICKNESS / 2.0 - OVERRUN, PIN_Z);
    let height = THICKNESS + 2.0 * OVERRUN;
    let (drill, _) =
        primitive_cylinder(m, Axis::new(start, Vec3::y())?, PIN_RADIUS, height, &Control::NONE)?;
    let (lug, _) = cut(m, plate, drill, &Control::NONE)?;
    Ok(lug)
}
