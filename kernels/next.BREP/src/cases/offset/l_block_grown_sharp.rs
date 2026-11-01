//! The L through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15), extruded 8 along +z, grown by
//! 1 with its edges kept sharp: every face moved 1 outward and the moved faces extended until they
//! meet, the L through (−1, −1), (21, −1), (21, 6), (6, 6), (6, 16), (−1, 16) from z = −1 to 9.
use crate::solids::offset::sharp;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let l = polygon(&[
        p(0.0, 0.0, 0.0), p(20.0, 0.0, 0.0), p(20.0, 5.0, 0.0),
        p(5.0, 5.0, 0.0), p(5.0, 15.0, 0.0), p(0.0, 15.0, 0.0),
    ])?;
    let block = extrude_profile_brep(&l, p(0.0, 0.0, 1.0), 8.0)?;
    Ok(sharp(&block, 1.0)?)
}
