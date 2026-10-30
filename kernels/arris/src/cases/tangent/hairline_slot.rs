//! A cube of side 10 at the origin with a saw cut 10⁻⁶ wide, x from 5 to 5.000001, right through
//! it in y and from z = 5 up through the top.
use arris::math::{Control, Point3};
use arris::ops::{cut, primitive_box, OpError};
use arris::topo::{Body, Model};

const SIDE: f64 = 10.0;
const SLOT_X: f64 = 5.0;
const SLOT_WIDTH: f64 = 1e-6;
const SLOT_FLOOR: f64 = 5.0;
/// How far the saw's block runs past the cube's front, back and top.
const OVERRUN: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (cube, _) =
        primitive_box(m, Point3::origin(), Point3::new(SIDE, SIDE, SIDE), &Control::NONE)?;
    let (saw, _) = primitive_box(
        m,
        Point3::new(SLOT_X, -OVERRUN, SLOT_FLOOR),
        Point3::new(SLOT_X + SLOT_WIDTH, SIDE + OVERRUN, SIDE + OVERRUN),
        &Control::NONE,
    )?;
    let (slotted, _) = cut(m, cube, saw, &Control::NONE)?;
    Ok(slotted)
}
