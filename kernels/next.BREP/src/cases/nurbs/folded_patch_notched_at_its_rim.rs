//! A block under a Bézier patch folded back over its own rim, its corner notched out by a box.
//!
//! The patch (degree 4 in u, 1 in v) is S(u, v) = (s², v, 1/2 + 3s⁴/10 + v/10), s = 2u − 1, folded
//! along u = 1/2 so that its part u ≤ 1/2 covers the face again. The block stands on z = 0 under the
//! part 13/20 ≤ u ≤ 1, its top a face on the patch cut down to that part, which the case allows
//! as the same face (`bezier_part`), less the box 1/20 ≤ x ≤ 1/2, −1 ≤ y ≤ 1/2, −1 ≤ z ≤ 3.
use crate::solids::p;
use crate::solids::sweep::{bezier_part, block_under, ruled_bezier_patch};
use crate::Refusal;
use brep_kernel::{boolean_operation, make_box_brep, BooleanOperation, BooleanOptions, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let patch = ruled_bezier_patch(&[1.0, 0.0, -1.0 / 3.0, 0.0, 1.0], &[0.8, 0.2, 0.8, 0.2, 0.8])?;
    let block = block_under(&bezier_part(&patch, 13.0 / 20.0, 0.0)?)?;
    let notch = make_box_brep(p(0.05, -1.0, -1.0), 0.45, 1.5, 4.0)?;
    Ok(boolean_operation(&block, &notch, BooleanOperation::Subtract, &BooleanOptions::default())?)
}
