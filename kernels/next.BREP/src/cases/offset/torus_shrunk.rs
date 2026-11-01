//! The solid torus about the z axis whose tube has radius 3 about a middle circle of radius 10,
//! shrunk by 1.5: the torus about the same middle circle with a tube of radius 1.5.
use crate::solids::offset::parallel;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_torus_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let torus = make_torus_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 10.0, 3.0)?;
    Ok(parallel(&torus, -1.5)?)
}
