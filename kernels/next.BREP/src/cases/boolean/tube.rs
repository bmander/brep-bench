//! A cylinder of radius 5 on the z axis from z = 0 to 10, bored along the axis by a hole of
//! radius 3: the drill a cylinder from z = −1 to z = 11, past both ends.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let outer = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 10.0)?;
    let drill = make_cylinder_brep(p(0.0, 0.0, -1.0), p(0.0, 0.0, 1.0), 3.0, 12.0)?;
    Ok(cut(&outer, &drill)?)
}
