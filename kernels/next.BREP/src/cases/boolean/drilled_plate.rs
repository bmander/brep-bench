//! A plate from the origin to (100, 20, 10), drilled along z by nine holes of radius 2 at y = 10
//! and x = 10, 20, … 90, one after another, each its own cut by a cylinder from z = −1 to 11.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let mut plate = make_box_brep(p(0.0, 0.0, 0.0), 100.0, 20.0, 10.0)?;
    for i in 1..=9 {
        let x = 10.0 * i as f64;
        let drill = make_cylinder_brep(p(x, 10.0, -1.0), p(0.0, 0.0, 1.0), 2.0, 12.0)?;
        plate = cut(&plate, &drill)?;
    }
    Ok(plate)
}
