//! A cylinder of radius 5 on the z axis, from z = 0 to z = 10, the circular edge around its top
//! face rounded to radius 1. The edge is selected by a point on it off the seam.
use crate::solids::fillet::on_circle;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cylinder = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 10.0)?;
    Ok(fillet_edges(&cylinder, &[on_circle(5.0, 10.0, 1.0)], None, 1.0, false, None)?)
}
