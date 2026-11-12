//! The plate from (0, −5, 0) to (20, 5, 4), less everything a square bar stood on a corner
//! (6 ≤ x ≤ 14, |y| + |z + 6| ≤ 1) passes through as it slides without turning by 8 along
//! (0.3, 0, √0.91), stopping inside the plate.
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the bar as a convex polyhedron, its
//! eight corners and six faces, slid by 8 along (0.3, 0, √0.91), and cut from the plate.
use crate::solids::p;
use crate::solids::swept::{box_tool, less_all, slide_by};
use crate::Refusal;
use brep_kernel::{make_box_brep, sweep_solid, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(0.0, -5.0, 0.0), 20.0, 10.0, 4.0)?;
    // the square on its corner at x = 6: from its lowest corner along its two lower sides
    let bar = box_tool(p(6.0, 0.0, -7.0), p(8.0, 0.0, 0.0), p(0.0, 1.0, 1.0), p(0.0, -1.0, 1.0));
    let slide = slide_by(p(0.3, 0.0, 0.91f64.sqrt()).scale(8.0));
    less_all(plate, vec![sweep_solid(&bar, &slide)?])
}
