//! The block from (0, 0, 0) to (20, 20, 6) with the square pocket from (6, 6) to (14, 14) cut
//! down from its top to a floor at z = 3, grown by 1 to its parallel body: its convex edges
//! rounded, the pocket's concave edges left sharp.
use crate::solids::boolean::cut;
use crate::solids::offset::parallel;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 20.0, 6.0)?;
    // the pocket's cutter runs out past the top
    let pocket = make_box_brep(p(6.0, 6.0, 3.0), 8.0, 8.0, 4.0)?;
    Ok(parallel(&cut(&block, &pocket)?, 1.0)?)
}
