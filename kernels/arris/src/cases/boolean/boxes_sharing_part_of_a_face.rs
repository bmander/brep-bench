//! Two cubes of side 10 joined, the second standing on half of the first's top.
use arris::math::{Control, Point3};
use arris::ops::{fuse, primitive_box, OpError};
use arris::topo::{Body, Model};

const SIDE: f64 = 10.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (lower, _) =
        primitive_box(m, Point3::origin(), Point3::new(SIDE, SIDE, SIDE), &Control::NONE)?;
    let (upper, _) = primitive_box(
        m,
        Point3::new(SIDE / 2.0, 0.0, SIDE),
        Point3::new(1.5 * SIDE, SIDE, 2.0 * SIDE),
        &Control::NONE,
    )?;
    let (joined, _) = fuse(m, lower, upper, &Control::NONE)?;
    Ok(joined)
}
