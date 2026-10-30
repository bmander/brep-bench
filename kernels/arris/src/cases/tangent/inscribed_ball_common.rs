//! The solid common to a cube of side 10 at the origin and its inscribed ball of radius 5 about
//! (5, 5, 5), which touches each face at its middle.
use crate::solids::ball;
use arris::math::{Control, Point3};
use arris::ops::{common, primitive_box, OpError};
use arris::topo::{Body, Model};

const SIDE: f64 = 10.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (cube, _) =
        primitive_box(m, Point3::origin(), Point3::new(SIDE, SIDE, SIDE), &Control::NONE)?;
    let centre = Point3::new(SIDE / 2.0, SIDE / 2.0, SIDE / 2.0);
    let (sphere, _) = ball(m, centre, SIDE / 2.0, &Control::NONE)?;
    let (shared, _) = common(m, cube, sphere, &Control::NONE)?;
    Ok(shared)
}
