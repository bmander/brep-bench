//! A cube of side 10 at the origin with a hole of radius 3 drilled through it along z.
use arris::math::{Axis, Control, Point3};
use arris::ops::{cut, primitive_box, primitive_cylinder, OpError};
use arris::topo::{Body, Model};

const SIDE: f64 = 10.0;
const HOLE_RADIUS: f64 = 3.0;
/// How far the drill runs past each face, so its ends lie clear of the cube's.
const OVERRUN: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (cube, _) =
        primitive_box(m, Point3::origin(), Point3::new(SIDE, SIDE, SIDE), &Control::NONE)?;
    let base = Point3::new(SIDE / 2.0, SIDE / 2.0, -OVERRUN);
    let height = SIDE + 2.0 * OVERRUN;
    let (drill, _) = primitive_cylinder(m, Axis::z_at(base), HOLE_RADIUS, height, &Control::NONE)?;
    let (drilled, _) = cut(m, cube, drill, &Control::NONE)?;
    Ok(drilled)
}
