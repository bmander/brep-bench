//! The L through (0, 0), (10, 0), (10, 3), (3, 3), (3, 10), (0, 10) of the xz plane, extruded
//! along +y from y = 0 to y = 10, its inside corner, the line x = 3, z = 3, rounded to radius 7,
//! adding material: the ball touches both faces at their far edges, so the round, a quarter of
//! the cylinder of radius 7 on the line x = 10, z = 10, consumes them.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let l = polygon(&[
        p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(10.0, 0.0, 3.0),
        p(3.0, 0.0, 3.0), p(3.0, 0.0, 10.0), p(0.0, 0.0, 10.0),
    ])?;
    let block = extrude_profile_brep(&l, p(0.0, 1.0, 0.0), 10.0)?;
    Ok(fillet_edges(&block, &[p(3.0, 5.0, 3.0)], None, 7.0, false, None)?)
}
