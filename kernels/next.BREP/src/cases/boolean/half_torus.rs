//! A torus about the z axis through the origin, centre circle of radius 10 and tube of radius 2,
//! with everything where x < 0 cut away by a block reaching past it.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_torus_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let torus = make_torus_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 10.0, 2.0)?;
    let left = make_box_brep(p(-20.0, -20.0, -5.0), 20.0, 40.0, 10.0)?;
    Ok(cut(&torus, &left)?)
}
