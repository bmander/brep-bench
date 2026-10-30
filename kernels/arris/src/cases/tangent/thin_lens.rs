//! The lens common to two balls of radius 5, centred on the origin and at (9.999999, 0, 0): 10⁻⁶
//! thick at its middle.
use crate::solids::ball;
use arris::math::{Control, Point3};
use arris::ops::{common, OpError};
use arris::topo::{Body, Model};

const RADIUS: f64 = 5.0;
const SPACING: f64 = 9.999999;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (first, _) = ball(m, Point3::origin(), RADIUS, &Control::NONE)?;
    let (second, _) = ball(m, Point3::new(SPACING, 0.0, 0.0), RADIUS, &Control::NONE)?;
    let (lens, _) = common(m, first, second, &Control::NONE)?;
    Ok(lens)
}
