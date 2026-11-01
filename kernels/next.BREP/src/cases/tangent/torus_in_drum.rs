//! A solid torus about the z axis, its tube's middle circle of radius 10 in the xy plane and its
//! tube of radius 3, united with a drum of radius 13 on the z axis from z = −1 to z = 1, whose wall
//! touches the torus along its outer equator.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, make_torus_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let torus = make_torus_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 10.0, 3.0)?;
    let drum = make_cylinder_brep(p(0.0, 0.0, -1.0), p(0.0, 0.0, 1.0), 13.0, 2.0)?;
    Ok(union(&torus, &drum)?)
}
