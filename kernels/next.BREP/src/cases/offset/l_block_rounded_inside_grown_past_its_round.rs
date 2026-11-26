//! The L through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15), extruded 8 along +z, its
//! inside corner's concave edge, along (5, 5), rounded to radius 4 by the kernel's fillet, then
//! grown by 5, more than the round's radius, to its parallel body.
use crate::solids::offset::parallel;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let l = polygon(&[
        p(0.0, 0.0, 0.0), p(20.0, 0.0, 0.0), p(20.0, 5.0, 0.0),
        p(5.0, 5.0, 0.0), p(5.0, 15.0, 0.0), p(0.0, 15.0, 0.0),
    ])?;
    let block = extrude_profile_brep(&l, p(0.0, 0.0, 1.0), 8.0)?;
    let rounded = fillet_edges(&block, &[p(5.0, 5.0, 4.0)], None, 4.0, false, None)?;
    Ok(parallel(&rounded, 5.0)?)
}
