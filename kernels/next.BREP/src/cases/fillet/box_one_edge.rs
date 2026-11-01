//! A cube of side 10 between the origin and (10, 10, 10), the edge where its faces x = 10 and
//! y = 10 meet rounded to radius 2. The edge is selected by its middle, (10, 10, 5).
use crate::solids::p;
use brep_kernel::{fillet_edges, make_box_brep, BrepSolid};
use crate::Refusal;

pub fn build() -> Result<BrepSolid, Refusal> {
    let cube = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 10.0, 10.0)?;
    Ok(fillet_edges(&cube, &[p(10.0, 10.0, 5.0)], None, 2.0, false, None)?)
}
