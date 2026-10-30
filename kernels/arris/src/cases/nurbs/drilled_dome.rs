//! The block under the Bézier dome of `nurbs/bezier-dome`, drilled straight down through by the
//! vertical cylinder of radius 3 about the line x = 12, y = 8.
use super::bezier_dome;
use crate::Refusal;
use arris::math::{Axis, Control, Point3};
use arris::ops::{cut, primitive_cylinder};
use arris::topo::{Body, Model};

const HOLE_X: f64 = 12.0;
const HOLE_Y: f64 = 8.0;
const HOLE_RADIUS: f64 = 3.0;
/// How far the drill runs past the floor and past the dome's highest control point (16).
const OVERRUN: f64 = 2.0;
const TOP: f64 = 16.0;

// workaround: no solid from a NURBS surface
pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let block = bezier_dome::build(m)?;
    let base = Point3::new(HOLE_X, HOLE_Y, -OVERRUN);
    let (drill, _) =
        primitive_cylinder(m, Axis::z_at(base), HOLE_RADIUS, TOP + 2.0 * OVERRUN, &Control::NONE)?;
    let (drilled, _) = cut(m, block, drill, &Control::NONE)?;
    Ok(drilled)
}
