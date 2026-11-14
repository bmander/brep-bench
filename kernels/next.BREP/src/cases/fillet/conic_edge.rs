//! The box from the origin to (20, 10, 10) with the edge where its top meets its end x = 20
//! blended by a conic of rho 0.5, a parabola, tangent to the top and the end each 3 from the edge.
//! The edge is selected by its middle, (20, 5, 10).
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges_conic, make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 10.0, 10.0)?;
    Ok(fillet_edges_conic(&block, &[p(20.0, 5.0, 10.0)], None, 3.0, 0.5, None)?)
}
