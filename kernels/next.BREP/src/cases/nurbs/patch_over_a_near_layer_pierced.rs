//! A block under a Bézier patch whose folded-back layer lies just beneath its face, pierced by a
//! square hole.
//!
//! The patch (degree 4 in u, 1 in v) is S(u, v) = (s², v, 1/2 + 3s⁴/10 + s³/1000 + v/10),
//! s = 2u − 1: its part u ≤ 1/2 lies a few ten-thousandths beneath its part u ≥ 1/2. The block
//! stands on z = 0 under the part 13/20 ≤ u ≤ 1, its top a face on the patch cut down to that part,
//! which the case allows as the same face (`bezier_part`), less the box 2/5 ≤ x ≤ 7/10, 3/10 ≤ y ≤ 3/5, −1 ≤ z ≤ 3.
use crate::solids::p;
use crate::solids::sweep::{bezier_part, block_under, ruled_bezier_patch};
use crate::Refusal;
use brep_kernel::{boolean_operation, make_box_brep, BooleanOperation, BooleanOptions, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let patch = ruled_bezier_patch(
        &[1.0, 0.0, -1.0 / 3.0, 0.0, 1.0],
        &[799.0 / 1000.0, 401.0 / 2000.0, 0.8, 399.0 / 2000.0, 801.0 / 1000.0],
    )?;
    let block = block_under(&bezier_part(&patch, 13.0 / 20.0, 0.0)?)?;
    let hole = make_box_brep(p(0.4, 0.3, -1.0), 0.3, 0.3, 4.0)?;
    Ok(boolean_operation(&block, &hole, BooleanOperation::Subtract, &BooleanOptions::default())?)
}
