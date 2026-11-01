//! The box from the origin to (10, 8, 6) shelled inward 3.99 thick, leaving a slot 0.02 wide.
use crate::solids::face_through;
use crate::Refusal;
use arris::math::{Control, Point3};
use arris::ops::{primitive_box, shell, ShellSide};
use arris::topo::{Body, Model};

const SIZE: (f64, f64, f64) = (10.0, 8.0, 6.0);
const THICKNESS: f64 = 3.99;

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let (b, _) =
        primitive_box(m, Point3::origin(), Point3::new(SIZE.0, SIZE.1, SIZE.2), &Control::NONE)?;
    let openings = [Point3::new(5.0, 4.0, 6.0)]
        .into_iter()
        .map(|p| face_through(m, b, p))
        .collect::<Result<Vec<_>, _>>()?;
    let (shelled, _) = shell(m, b, &openings, THICKNESS, ShellSide::Inward, &Control::NONE)?;
    Ok(shelled)
}
