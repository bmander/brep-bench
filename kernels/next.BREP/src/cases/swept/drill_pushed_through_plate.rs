//! The plate from (0, −5, 0) to (20, 5, 4), less everything a drill passes through as it slides
//! straight down its own axis by 20: about the upright line through (10, 0), a cylinder of radius 2
//! from z = 6 + 2/tan 59° to 14, ended below by a cone of 118° with its point at (10, 0, 6).
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the drill, a solid of revolution
//! about the upright line through (10, 0, 0) whose meridian is the cone's side from its point, the
//! cylinder's side and the flat top, slid 20 straight down, and cut from the plate.
use crate::solids::p;
use crate::solids::swept::{less_all, revolved_tool, slide_by, Piece};
use crate::Refusal;
use brep_kernel::{make_box_brep, sweep_solid, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let plate = make_box_brep(p(0.0, -5.0, 0.0), 20.0, 10.0, 4.0)?;
    let shoulder = 6.0 + 2.0 / 59f64.to_radians().tan();
    let drill = revolved_tool(p(10.0, 0.0, 0.0), z, x, &[
        Piece::Line { from: [0.0, 6.0], to: [2.0, shoulder] },
        Piece::Line { from: [2.0, shoulder], to: [2.0, 14.0] },
        Piece::Line { from: [2.0, 14.0], to: [0.0, 14.0] },
        Piece::Line { from: [0.0, 14.0], to: [0.0, 6.0] },
    ]);
    less_all(plate, vec![sweep_solid(&drill, &slide_by(p(0.0, 0.0, -20.0)))?])
}
