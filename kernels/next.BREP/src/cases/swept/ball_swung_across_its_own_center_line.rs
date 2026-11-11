//! The solid a ball fills as it is turned about a line through it.
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the ball, a solid of revolution
//! about the vertical line through its centre (1, 0, 0), its meridian the half disc of radius 2;
//! turned about the z axis from 0° to 120°. The turn's axis passes through the ball, so the kernel
//! bounds the sweep by the outer half of its grazing circle turned, the ball's back at the start
//! less the ball at the end, and its front at the end less the ball at the start.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{sweep_solid, BrepSolid, OutlinePiece, ScrewStep, SolidMotion, SolidTool};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = SolidTool::Revolved {
        axis_origin: p(1.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        radial: p(1.0, 0.0, 0.0),
        meridian: vec![
            OutlinePiece::Arc { centre: [0.0, 0.0], radius: 2.0, start: -FRAC_PI_2, end: FRAC_PI_2 },
            OutlinePiece::Line { from: [0.0, 2.0], to: [0.0, -2.0] },
        ],
    };
    let motion = SolidMotion {
        steps: vec![ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 0.0 }],
        from: 0.0,
        to: 120f64.to_radians(),
    };
    Ok(sweep_solid(&ball, &motion)?)
}
