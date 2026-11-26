//! The plate from (0, 0, 0) to (10, 10, 2) with the square hole from (3, 3) to (7, 7) through it,
//! grown by 1 to its parallel body: its convex edges rounded, the hole's four concave vertical
//! edges left sharp.
use crate::solids::boolean::cut;
use crate::solids::offset::parallel;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 10.0, 2.0)?;
    // the hole's cutter runs past both faces
    let hole = make_box_brep(p(3.0, 3.0, -1.0), 4.0, 4.0, 4.0)?;
    Ok(parallel(&cut(&plate, &hole)?, 1.0)?)
}
