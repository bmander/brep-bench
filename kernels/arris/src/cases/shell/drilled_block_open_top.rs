//! The block from the origin to (20, 20, 10) bored through along x = 10, y = 10 with a hole of
//! radius 3, shelled inward 1 thick with its top open: the bore becomes a tube in the cup.
use crate::solids::face_through;
use crate::Refusal;
use arris::math::{Axis, Control, Point3};
use arris::ops::{cut, primitive_box, primitive_cylinder, shell, ShellSide};
use arris::topo::{Body, Model};

const AXIS: (f64, f64) = (10.0, 10.0);

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let block = drilled(m, AXIS.0, AXIS.1)?;
    let opening = face_through(m, block, Point3::new(1.0, 1.0, SIZE.2))?;
    let (shelled, _) =
        shell(m, block, &[opening], THICKNESS, ShellSide::Inward, &Control::NONE)?;
    Ok(shelled)
}

const SIZE: (f64, f64, f64) = (20.0, 20.0, 10.0);
const HOLE_RADIUS: f64 = 3.0;
const THICKNESS: f64 = 1.0;
/// How far the bore's cylinder runs past each face, so its ends lie clear of the block's.
const OVERRUN: f64 = 1.0;

/// The block with the hole bored along z through (x, y).
fn drilled(m: &mut Model, x: f64, y: f64) -> Result<Body, Refusal> {
    let (w, d, h) = SIZE;
    let (block, _) = primitive_box(m, Point3::origin(), Point3::new(w, d, h), &Control::NONE)?;
    let base = Point3::new(x, y, -OVERRUN);
    let (bore, _) =
        primitive_cylinder(m, Axis::z_at(base), HOLE_RADIUS, h + 2.0 * OVERRUN, &Control::NONE)?;
    let (drilled, _) = cut(m, block, bore, &Control::NONE)?;
    Ok(drilled)
}
