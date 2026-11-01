//! A cylinder of radius 5 on the z axis, from z = 0 to z = 10, the circular edge around its top
//! face rounded to radius 5, the cylinder's own: the top face shrinks to the point (0, 0, 10) and
//! the round is the hemisphere of radius 5 about (0, 0, 5).
use crate::solids::fillet::on_circle;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cylinder = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 10.0)?;
    Ok(fillet_edges(&cylinder, &[on_circle(5.0, 10.0, 1.0)], None, 5.0, false, None)?)
}
