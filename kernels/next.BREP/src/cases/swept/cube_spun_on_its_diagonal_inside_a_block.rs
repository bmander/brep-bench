//! The block from (−3, −3, −3) to (3, 3, 3), less everything the cube from (−1, −1, −1) to
//! (1, 1, 1) passes through as it turns a whole turn about its long diagonal.
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the cube as a convex polyhedron,
//! its eight corners and six faces, turned a whole turn about the line through the origin along (1,
//! 1, 1), and cut from the block.
use crate::solids::p;
use crate::solids::swept::{box_tool, less_all};
use crate::Refusal;
use brep_kernel::{make_box_brep, sweep_solid, BrepSolid, ScrewStep, SolidMotion};
use std::f64::consts::{TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(-3.0, -3.0, -3.0), 6.0, 6.0, 6.0)?;
    let cube = box_tool(p(-1.0, -1.0, -1.0), p(2.0, 0.0, 0.0), p(0.0, 2.0, 0.0), p(0.0, 0.0, 2.0));
    let turn = SolidMotion {
        steps: vec![ScrewStep {
            axis_origin: p(0.0, 0.0, 0.0),
            axis_direction: p(1.0, 1.0, 1.0).scale(1.0 / 3f64.sqrt()),
            turn: 1.0,
            advance: 0.0,
        }],
        from: 0.0,
        to: TAU,
    };
    less_all(block, vec![sweep_solid(&cube, &turn)?])
}
