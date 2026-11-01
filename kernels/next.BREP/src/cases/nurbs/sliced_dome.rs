//! The block under the bicubic Bézier dome of `nurbs/bezier-dome`, with everything above the
//! inclined plane z = 4 + x/2 cut away: the block split by that plane, the piece below it kept.
use crate::solids::p;
use crate::solids::sweep::block_under;
use crate::Refusal;
use brep_kernel::{split_solid_by_plane, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = block_under(&super::bezier_dome::dome()?)?;
    // the plane through the corner (0, 0, 4), its normal (−1/2, 0, 1) pointing up and away
    let (below, _above) = split_solid_by_plane(&block, p(0.0, 0.0, 4.0), p(-0.5, 0.0, 1.0))?;
    Ok(below)
}
