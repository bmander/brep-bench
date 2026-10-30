//! A solid ball of radius 5 centred on the origin.
use crate::solids::ball;
use arris::math::{Control, Point3};
use arris::ops::OpError;
use arris::topo::{Body, Model};

const RADIUS: f64 = 5.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    // arris has no sphere primitive: `ball` turns a half disc about its diameter
    let (sphere, _) = ball(m, Point3::origin(), RADIUS, &Control::NONE)?;
    Ok(sphere)
}
