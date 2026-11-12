//! The box from (−6, −6, −8) to (18, 18, 0), less everything a ball of radius 3 passes through as
//! its centre swings a quarter turn about the z axis, right-handed, from (12, 0, 0) to (0, 12, 0).
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the ball, a solid of revolution
//! about the upright line through (12, 0, 0) whose meridian is the half disc of radius 3, turned a
//! quarter turn about the z axis, and cut from the block.
use crate::solids::p;
use crate::solids::swept::{less_all, revolved_tool, turn_about_z, Piece};
use crate::Refusal;
use brep_kernel::{make_box_brep, sweep_solid, BrepSolid};
use std::f64::consts::{FRAC_PI_2};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let block = make_box_brep(p(-6.0, -6.0, -8.0), 24.0, 24.0, 8.0)?;
    let ball = revolved_tool(p(12.0, 0.0, 0.0), z, x, &[
        Piece::Arc { centre: [0.0, 0.0], radius: 3.0, start: -FRAC_PI_2, end: FRAC_PI_2 },
        Piece::Line { from: [0.0, 3.0], to: [0.0, -3.0] },
    ]);
    less_all(block, vec![sweep_solid(&ball, &turn_about_z(0.0, FRAC_PI_2, 0.0))?])
}
