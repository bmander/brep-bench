//! The block from (0, −5, 0) to (20, 5, 10), less everything a V wheel passes through as it slides
//! without turning by 36 along x: the solid of revolution, about the line through (−8, 0, 14)
//! along y, of the triangle with corners (y, z) = (0, 8) and (±3 tan 30°, 11).
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the wheel, a solid of revolution
//! about the line through (−8, 0, 14) along y whose meridian is the triangle with its point 6 from
//! the axis and its base 3 from it, slid 36 along x, and cut from the block.
use crate::solids::p;
use crate::solids::swept::{less_all, revolved_tool, slide_by, Piece};
use crate::Refusal;
use brep_kernel::{make_box_brep, sweep_solid, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, -5.0, 0.0), 20.0, 10.0, 10.0)?;
    let r3 = 3f64.sqrt();
    // the triangle in (distance from the axis, toward −z; place along y)
    let wheel = revolved_tool(p(-8.0, 0.0, 14.0), p(0.0, 1.0, 0.0), p(0.0, 0.0, -1.0), &[
        Piece::Line { from: [3.0, -r3], to: [6.0, 0.0] },
        Piece::Line { from: [6.0, 0.0], to: [3.0, r3] },
        Piece::Line { from: [3.0, r3], to: [3.0, -r3] },
    ]);
    less_all(block, vec![sweep_solid(&wheel, &slide_by(p(36.0, 0.0, 0.0)))?])
}
