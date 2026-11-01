//! A block from the origin to (10, 4, 6), the two edges of its top face that run along x, at
//! y = 0 and y = 4, chamfered with distance 2 on both faces, half the top's width, in one
//! chamfer: the chamfers take the whole top face and meet at right angles along the ridge y = 2,
//! z = 6.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 4.0, 6.0)?;
    // The kernel's symmetric chamfer is the chord of the rolling ball's arc; on a right-angled
    // edge its distance is the setback on each face.
    let edges = [p(5.0, 0.0, 6.0), p(5.0, 4.0, 6.0)];
    Ok(fillet_edges(&block, &edges, None, 2.0, true, None)?)
}
