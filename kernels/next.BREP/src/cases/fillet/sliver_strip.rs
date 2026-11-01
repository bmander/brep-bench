//! A plate from the origin to (20, 10, 2), the two edges of its face y = 10 that run along x, at
//! z = 0 and z = 2, rounded to radius 0.9999 in one fillet, leaving of that face a strip 0.0002
//! tall between the rounds.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 10.0, 2.0)?;
    let edges = [p(10.0, 10.0, 0.0), p(10.0, 10.0, 2.0)];
    Ok(fillet_edges(&plate, &edges, None, 0.9999, false, None)?)
}
