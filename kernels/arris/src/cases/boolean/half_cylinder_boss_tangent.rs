//! A cube of side 10 united with a cylinder of radius 5 half inside it, tangent to two faces.
use arris::math::{Axis, Control, Point3};
use arris::ops::{fuse, primitive_box, primitive_cylinder, OpError};
use arris::topo::{Body, Model};

const SIDE: f64 = 10.0;
const BOSS_RADIUS: f64 = 5.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (cube, _) =
        primitive_box(m, Point3::origin(), Point3::new(SIDE, SIDE, SIDE), &Control::NONE)?;
    let axis = Axis::z_at(Point3::new(SIDE, SIDE / 2.0, 0.0));
    let (boss, _) = primitive_cylinder(m, axis, BOSS_RADIUS, SIDE, &Control::NONE)?;
    let (bossed, _) = fuse(m, cube, boss, &Control::NONE)?;
    Ok(bossed)
}
