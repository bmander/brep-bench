//! A block with a ball of radius 5 cut from it, a hemispherical pocket in its top face.
use crate::solids::ball;
use arris::math::{Control, Point3};
use arris::ops::{cut, primitive_box, OpError};
use arris::topo::{Body, Model};

const HALF_WIDTH: f64 = 10.0;
const DEPTH: f64 = 10.0;
const RADIUS: f64 = 5.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (block, _) = primitive_box(
        m,
        Point3::new(-HALF_WIDTH, -HALF_WIDTH, -DEPTH),
        Point3::new(HALF_WIDTH, HALF_WIDTH, 0.0),
        &Control::NONE,
    )?;
    let (sphere, _) = ball(m, Point3::origin(), RADIUS, &Control::NONE)?;
    let (pocketed, _) = cut(m, block, sphere, &Control::NONE)?;
    Ok(pocketed)
}
