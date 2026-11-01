//! The L through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15), extruded 8 along +z, shrunk
//! by 1: every point at least 1 from its outside, its convex edges sharp and its inside corner's
//! concave edge a concave quarter cylinder of radius 1.
use crate::solids::offset::parallel;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let l = polygon(&[
        p(0.0, 0.0, 0.0), p(20.0, 0.0, 0.0), p(20.0, 5.0, 0.0),
        p(5.0, 5.0, 0.0), p(5.0, 15.0, 0.0), p(0.0, 15.0, 0.0),
    ])?;
    let block = extrude_profile_brep(&l, p(0.0, 0.0, 1.0), 8.0)?;
    Ok(parallel(&block, -1.0)?)
}
