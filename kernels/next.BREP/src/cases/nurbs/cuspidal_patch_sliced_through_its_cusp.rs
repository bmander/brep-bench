//! A block under a cuspidal Bézier patch, sliced by the plane z = 11y/10 through the cusp beside it.
//!
//! The patch (degree 3 in u, 1 in v) is S(u, v) = (s², v, 1/2 + s³ + v/10), s = 2u − 1, cuspidal
//! along u = 1/2. The block stands on z = 0 under the patch's part 11/20 ≤ u ≤ 1, its top a face
//! on the patch cut down to that part, which the case allows as the same face (`bezier_part`), and the part on or below the
//! plane is kept (`split_solid_by_plane`).
use crate::solids::p;
use crate::solids::sweep::{bezier_part, block_under, ruled_bezier_patch};
use crate::Refusal;
use brep_kernel::{split_solid_by_plane, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let third = 1.0 / 3.0;
    let patch = ruled_bezier_patch(&[1.0, -third, -third, 1.0], &[-0.5, 1.5, -0.5, 1.5])?;
    let block = block_under(&bezier_part(&patch, 11.0 / 20.0, 0.0)?)?;
    // the plane z = 11y/10 through the origin, its normal (0, −11/10, 1) pointing up and away
    let (below, _above) = split_solid_by_plane(&block, p(0.0, 0.0, 0.0), p(0.0, -1.1, 1.0))?;
    Ok(below)
}
