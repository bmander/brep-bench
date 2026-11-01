//! A square plate from (−20, −20, 0) to (20, 20, 4) and a boss on it, the cylinder of radius 5 on
//! the z axis from z = 4 to z = 8, joined into one solid; both of the boss's circular edges, its
//! rim at z = 8 (convex) and its root at z = 4 (concave), rounded to radius 2 in one fillet. The
//! rounds take the boss's whole side and meet, tangent, on the circle of radius 5 at z = 6.
use crate::solids::fillet::{on_circle, union};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(-20.0, -20.0, 0.0), 40.0, 40.0, 4.0)?;
    let boss = make_cylinder_brep(p(0.0, 0.0, 4.0), p(0.0, 0.0, 1.0), 5.0, 4.0)?;
    let joined = union(&plate, &boss)?;
    let edges = [on_circle(5.0, 8.0, 1.0), on_circle(5.0, 4.0, 1.0)];
    Ok(fillet_edges(&joined, &edges, None, 2.0, false, None)?)
}
