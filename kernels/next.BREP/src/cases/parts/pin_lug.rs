//! A flat lug, the plate from (0, −5, 0) to (80, 5, 40), with a pin hole of radius 8 through its
//! thickness along y on the line x = 60, z = 20, cut by a cylinder running past both faces.
use crate::solids::fillet::subtract;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(0.0, -5.0, 0.0), 80.0, 10.0, 40.0)?;
    let pin = make_cylinder_brep(p(60.0, -6.0, 20.0), p(0.0, 1.0, 0.0), 8.0, 12.0)?;
    Ok(subtract(&plate, &pin)?)
}
