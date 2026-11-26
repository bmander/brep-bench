//! The solid torus about the z axis whose tube has radius 2 about a middle circle of radius 4,
//! grown by 2 to its parallel body: every point within 4 of the middle circle, a horn torus.
use crate::solids::offset::parallel;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_torus_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let torus = make_torus_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 4.0, 2.0)?;
    Ok(parallel(&torus, 2.0)?)
}
