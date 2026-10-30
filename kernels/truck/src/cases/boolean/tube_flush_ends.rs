//! "A cylinder of radius 5 on the z axis, from z = 0 to z = 10. Cut from it a cylinder of radius 3
//! on the same axis that also runs exactly from z = 0 to z = 10. The ends of the cutting cylinder
//! lie in the ends of the first."
use crate::{shapes, Built, Refused};
use truck_modeling::*;

const OUTER_RADIUS: f64 = 5.0;
const INNER_RADIUS: f64 = 3.0;
const LENGTH: f64 = 10.0;

pub fn build() -> Built {
    let rod = shapes::cylinder(Point3::origin(), Vector3::unit_z(), OUTER_RADIUS, LENGTH);
    let mut core = shapes::cylinder(Point3::origin(), Vector3::unit_z(), INNER_RADIUS, LENGTH);
    // truck-shapeops cuts by intersecting with the tool turned inside out
    core.not();
    let tolerance = shapes::boolean_tolerance(&rod, &core);
    let tube = truck_shapeops::and(&rod, &core, tolerance)
        .ok_or_else(|| Refused::none_from("truck-shapeops `and`"))?;
    Ok(tube.into())
}
