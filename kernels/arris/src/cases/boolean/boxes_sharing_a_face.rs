//! Two cubes of side 10 side by side, joined into one: the face they share disappears.
use arris::math::{Control, Point3};
use arris::ops::{fuse, primitive_box, OpError};
use arris::topo::{Body, Model};

const SIDE: f64 = 10.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (left, _) =
        primitive_box(m, Point3::origin(), Point3::new(SIDE, SIDE, SIDE), &Control::NONE)?;
    let (right, _) = primitive_box(
        m,
        Point3::new(SIDE, 0.0, 0.0),
        Point3::new(2.0 * SIDE, SIDE, SIDE),
        &Control::NONE,
    )?;
    let (joined, _) = fuse(m, left, right, &Control::NONE)?;
    Ok(joined)
}
