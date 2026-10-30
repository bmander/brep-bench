//! A cylinder of radius 5 less one of radius 3 over exactly the same span of the z axis.
use arris::math::{Axis, Control, Point3};
use arris::ops::{cut, primitive_cylinder, OpError};
use arris::topo::{Body, Model};

const RADIUS: f64 = 5.0;
const HEIGHT: f64 = 10.0;
const BORE: f64 = 3.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let axis = Axis::z_at(Point3::origin());
    let (rod, _) = primitive_cylinder(m, axis, RADIUS, HEIGHT, &Control::NONE)?;
    // the tool's caps lie in the rod's: arris finds the coincident faces and drops them
    let (core, _) = primitive_cylinder(m, axis, BORE, HEIGHT, &Control::NONE)?;
    let (tube, _) = cut(m, rod, core, &Control::NONE)?;
    Ok(tube)
}
