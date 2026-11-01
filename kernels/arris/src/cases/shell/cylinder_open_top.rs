//! A cylinder of radius 5 on the z axis, from z = 0 to 10, shelled inward 1 thick, open at its top:
//! a cup.
use crate::solids::face_through;
use crate::Refusal;
use arris::math::{Axis, Control, Point3};
use arris::ops::{primitive_cylinder, shell, ShellSide};
use arris::topo::{Body, Model};

const RADIUS: f64 = 5.0;
const HEIGHT: f64 = 10.0;
const THICKNESS: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let (c, _) =
        primitive_cylinder(m, Axis::z_at(Point3::origin()), RADIUS, HEIGHT, &Control::NONE)?;
    let opening = face_through(m, c, Point3::new(0.0, 0.0, HEIGHT))?;
    let (shelled, _) = shell(m, c, &[opening], THICKNESS, ShellSide::Inward, &Control::NONE)?;
    Ok(shelled)
}
