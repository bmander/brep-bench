//! The plate from (−15, −15, 0) to (15, 15, 4), less everything a tilted box passes through as it
//! slides without turning along (4, 2, 18): the box (−2, −1, −8) + a e₁ + b e₂ + c e₃, |a| ≤ 2,
//! |b| ≤ 1.5, |c| ≤ 1, e₁ = (2, 1, 2)/3, e₂ = (−2, 2, 1)/3, e₃ = (1, 2, −2)/3.
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the box as a convex polyhedron, its
//! eight corners and six faces, slid by (4, 2, 18), and cut from the plate.
use crate::solids::p;
use crate::solids::swept::{box_tool, less_all, slide_by};
use crate::Refusal;
use brep_kernel::{make_box_brep, sweep_solid, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(-15.0, -15.0, 0.0), 30.0, 30.0, 4.0)?;
    let third = |x: f64, y: f64, z: f64| p(x, y, z).scale(1.0 / 3.0);
    let (e1, e2, e3) = (third(2.0, 1.0, 2.0), third(-2.0, 2.0, 1.0), third(1.0, 2.0, -2.0));
    let corner = p(-2.0, -1.0, -8.0).add(e1.scale(-2.0)).add(e2.scale(-1.5)).add(e3.scale(-1.0));
    let tool = box_tool(corner, e1.scale(4.0), e2.scale(3.0), e3.scale(2.0));
    less_all(plate, vec![sweep_solid(&tool, &slide_by(p(4.0, 2.0, 18.0)))?])
}
