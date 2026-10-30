//! A plate 100 by 20 by 10 drilled with nine holes of radius 2, each hole its own cut.
use arris::math::{Axis, Control, Point3};
use arris::ops::{cut, primitive_box, primitive_cylinder, OpError};
use arris::topo::{Body, Model};

const LENGTH: f64 = 100.0;
const WIDTH: f64 = 20.0;
const THICKNESS: f64 = 10.0;
const HOLES: usize = 9;
const PITCH: f64 = 10.0;
const HOLE_RADIUS: f64 = 2.0;
/// How far each drill runs past the plate's faces, so its ends lie clear of them.
const OVERRUN: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (mut plate, _) =
        primitive_box(m, Point3::origin(), Point3::new(LENGTH, WIDTH, THICKNESS), &Control::NONE)?;
    for k in 1..=HOLES {
        let base = Point3::new(PITCH * k as f64, WIDTH / 2.0, -OVERRUN);
        let height = THICKNESS + 2.0 * OVERRUN;
        let (drill, _) =
            primitive_cylinder(m, Axis::z_at(base), HOLE_RADIUS, height, &Control::NONE)?;
        (plate, _) = cut(m, plate, drill, &Control::NONE)?;
    }
    Ok(plate)
}
