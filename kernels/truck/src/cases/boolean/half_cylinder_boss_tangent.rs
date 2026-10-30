//! "Take a cube of side 10 between the origin and (10, 10, 10). Unite it with a cylinder of radius
//! 5 and height 10 that stands on the xy plane, its axis at x = 10, y = 5. Half of the cylinder
//! lies inside the cube. Its surface touches the cube's faces y = 0 and y = 10 exactly along the
//! cube's edges at x = 10."
use crate::{shapes, Built, Refused};
use truck_modeling::*;

const SIDE: f64 = 10.0;
const BOSS_RADIUS: f64 = 5.0;
const BOSS_HEIGHT: f64 = 10.0;
const BOSS_AXIS: (f64, f64) = (10.0, 5.0); // x, y

pub fn build() -> Built {
    let corners = [Point3::origin(), Point3::new(SIDE, SIDE, SIDE)];
    let cube: Solid = primitive::cuboid(BoundingBox::from_iter(corners));
    let foot = Point3::new(BOSS_AXIS.0, BOSS_AXIS.1, 0.0);
    let boss = shapes::cylinder(foot, Vector3::unit_z(), BOSS_RADIUS, BOSS_HEIGHT);
    let tolerance = shapes::boolean_tolerance(&cube, &boss);
    let united = truck_shapeops::or(&cube, &boss, tolerance)
        .ok_or_else(|| Refused::none_from("truck-shapeops `or`"))?;
    Ok(united.into())
}
