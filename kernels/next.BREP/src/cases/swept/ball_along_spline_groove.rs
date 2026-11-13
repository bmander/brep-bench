//! The box from (−6, −6, −8) to (36, 26, 0), less everything a ball of radius 3 passes through as
//! its centre follows the cubic Bézier curve in its top face with control points (0, 0, 0),
//! (10, 0, 0), (20, 20, 0) and (30, 20, 0).
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the ball, a solid of revolution
//! whose meridian is the half disc of radius 3, carried along the curve by the frame riding it
//! (`sweep_solid_along`), and cut from the block.
use crate::solids::p;
use crate::solids::swept::{less_all, revolved_tool, Piece};
use crate::Refusal;
use brep_kernel::{make_box_brep, sweep_solid_along, BrepSolid, FrameCurve};
use std::f64::consts::{FRAC_PI_2};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let block = make_box_brep(p(-6.0, -6.0, -8.0), 42.0, 32.0, 8.0)?;
    let ball = revolved_tool(p(0.0, 0.0, 0.0), z, x, &[
        Piece::Arc { centre: [0.0, 0.0], radius: 3.0, start: -FRAC_PI_2, end: FRAC_PI_2 },
        Piece::Line { from: [0.0, 3.0], to: [0.0, -3.0] },
    ]);
    let path = FrameCurve {
        control: [p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(20.0, 20.0, 0.0), p(30.0, 20.0, 0.0)],
        up: z,
    };
    less_all(block, vec![sweep_solid_along(&ball, &path)?])
}
