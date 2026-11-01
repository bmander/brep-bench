//! A ball of radius 5 centred on the origin, hollowed to a wall 1 thick with no opening.
use crate::solids::ball;
use crate::Refusal;
use arris::math::{Control, Point3};
use arris::ops::{shell, ShellSide};
use arris::topo::{Body, Model};

const RADIUS: f64 = 5.0;
const THICKNESS: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    // arris has no sphere primitive: `ball` turns a half disc about its diameter
    let (sphere, _) = ball(m, Point3::origin(), RADIUS, &Control::NONE)?;
    // no openings: the cavity is a closed void, a second shell inside the first
    let (hollow, _) = shell(m, sphere, &[], THICKNESS, ShellSide::Inward, &Control::NONE)?;
    Ok(hollow)
}
