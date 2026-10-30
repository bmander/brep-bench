//! The union of two balls of radius 5, centred on the origin and at (6, 0, 0).
use crate::solids::ball;
use arris::math::{Control, Point3};
use arris::ops::{fuse, OpError};
use arris::topo::{Body, Model};

const RADIUS: f64 = 5.0;
const SPACING: f64 = 6.0;

// workaround: no sphere primitive
pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (first, _) = ball(m, Point3::origin(), RADIUS, &Control::NONE)?;
    let (second, _) = ball(m, Point3::new(SPACING, 0.0, 0.0), RADIUS, &Control::NONE)?;
    let (joined, _) = fuse(m, first, second, &Control::NONE)?;
    Ok(joined)
}
