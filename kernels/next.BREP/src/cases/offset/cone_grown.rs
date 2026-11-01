//! The cone on the z axis with a base of radius 3 at z = 0 and its apex at (0, 0, 4), grown by 1
//! to its parallel body: its side moved out, its rim a piece of a torus, its apex a cap of a ball.
use crate::solids::offset::parallel;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cone_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cone = make_cone_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 3.0, 0.0, 4.0)?;
    Ok(parallel(&cone, 1.0)?)
}
