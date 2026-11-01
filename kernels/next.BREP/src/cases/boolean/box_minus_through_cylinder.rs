//! A cube of side 10 from the origin to (10, 10, 10), drilled through along z by a hole of radius
//! 3 centred at x = 5, y = 5: the drill a cylinder from z = −1 to z = 11, past both faces.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cube = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 10.0, 10.0)?;
    let drill = make_cylinder_brep(p(5.0, 5.0, -1.0), p(0.0, 0.0, 1.0), 3.0, 12.0)?;
    Ok(cut(&cube, &drill)?)
}
