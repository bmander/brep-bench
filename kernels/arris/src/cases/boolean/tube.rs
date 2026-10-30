//! A cylinder of radius 5 from z = 0 to 10 with a hole of radius 3 bored along its axis.
use arris::math::{Axis, Control, Point3};
use arris::ops::{cut, primitive_cylinder, OpError};
use arris::topo::{Body, Model};

const RADIUS: f64 = 5.0;
const HEIGHT: f64 = 10.0;
const BORE: f64 = 3.0;
/// How far the bore runs past each end, so its ends lie clear of the cylinder's.
const OVERRUN: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (rod, _) =
        primitive_cylinder(m, Axis::z_at(Point3::origin()), RADIUS, HEIGHT, &Control::NONE)?;
    let base = Point3::new(0.0, 0.0, -OVERRUN);
    let (bore, _) =
        primitive_cylinder(m, Axis::z_at(base), BORE, HEIGHT + 2.0 * OVERRUN, &Control::NONE)?;
    let (tube, _) = cut(m, rod, bore, &Control::NONE)?;
    Ok(tube)
}
