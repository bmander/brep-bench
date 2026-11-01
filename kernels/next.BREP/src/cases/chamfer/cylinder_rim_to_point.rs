//! A cylinder of radius 5 on the z axis, from z = 0 to z = 10, the circular edge around its top
//! face chamfered with distance 5 on both faces, the cylinder's own radius: the top face shrinks
//! to its centre and the chamfer is the cone from the circle of radius 5 at z = 5 to the apex
//! (0, 0, 10).
use crate::solids::fillet::on_circle;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cylinder = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 10.0)?;
    // The rim is a right-angled edge, so the chamfer's distance is its setback on both faces.
    Ok(fillet_edges(&cylinder, &[on_circle(5.0, 10.0, 1.0)], None, 5.0, true, None)?)
}
