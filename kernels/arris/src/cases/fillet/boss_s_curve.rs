//! A plate from (−20, −20, 0) to (20, 20, 4) with a boss of radius 5 on the z axis from z = 4 to
//! 8, the boss's rim and its root each rounded to radius 2, so its side vanishes into an S.
use arris::math::{Axis, Control, Point3};
use arris::ops::{fillet, fuse, primitive_box, primitive_cylinder, OpError};
use arris::topo::provenance::{CylinderPart, Role};
use arris::topo::{Body, Edge, Model};

const HALF_WIDTH: f64 = 20.0;
const PLATE_THICKNESS: f64 = 4.0;
const BOSS_RADIUS: f64 = 5.0;
const BOSS_HEIGHT: f64 = 4.0;
const ROUND: f64 = 2.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (plate, _) = primitive_box(
        m,
        Point3::new(-HALF_WIDTH, -HALF_WIDTH, 0.0),
        Point3::new(HALF_WIDTH, HALF_WIDTH, PLATE_THICKNESS),
        &Control::NONE,
    )?;
    // the boss stands on the plate's top: arris finds its bottom cap coincident and drops it
    let base = Point3::new(0.0, 0.0, PLATE_THICKNESS);
    let (boss, made) =
        primitive_cylinder(m, Axis::z_at(base), BOSS_RADIUS, BOSS_HEIGHT, &Control::NONE)?;
    let (bossed, _) = fuse(m, plate, boss, &Control::NONE)?;
    // the rim, and the root: the boss's bottom rim, where its wall meets the plate's top once the
    // coincident cap is gone; the fuse keeps both circles by id. Both rounded in one blend.
    let circles: Vec<Edge> = [CylinderPart::TopRim, CylinderPart::BottomRim]
        .into_iter()
        .flat_map(|part| made.generated_from(Role::Cylinder(part)).to_vec())
        .filter_map(|s| Edge::try_from(s).ok())
        .collect();
    let (rounded, _) = fillet(m, bossed, &circles, ROUND, &Control::NONE)?;
    Ok(rounded)
}
