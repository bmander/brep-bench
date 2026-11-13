//! The block from (0, −10, −10) to (30, 10, 0), less everything a V bit passes through as it slides
//! without turning along x by 15: the cone about the upright line through (5, 0) with its point at
//! (5, 0, −2), opening at 90° to a flat top of radius 5 at z = 3.
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the bit, a solid of revolution
//! about the upright line through (5, 0, 0) whose meridian is the cone's side from its point at z =
//! −2 out to radius 5 at z = 3 and the flat top, slid 15 along x, and cut from the block.
use crate::solids::p;
use crate::solids::swept::{less_all, revolved_tool, slide_by, Piece};
use crate::Refusal;
use brep_kernel::{make_box_brep, sweep_solid, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let block = make_box_brep(p(0.0, -10.0, -10.0), 30.0, 20.0, 10.0)?;
    let bit = revolved_tool(p(5.0, 0.0, 0.0), z, x, &[
        Piece::Line { from: [0.0, -2.0], to: [5.0, 3.0] },
        Piece::Line { from: [5.0, 3.0], to: [0.0, 3.0] },
        Piece::Line { from: [0.0, 3.0], to: [0.0, -2.0] },
    ]);
    less_all(block, vec![sweep_solid(&bit, &slide_by(p(15.0, 0.0, 0.0)))?])
}
