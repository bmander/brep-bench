//! A ring sector grooved by a drum screwed across it.
//!
//! The kernel's whole sweep of a solid (`sweep_solid`): the drum is a solid of revolution about
//! the vertical line through (10, 0), its meridian the rectangle out to radius 1 from z = −0.5 to
//! 0.5, swept by the screw about the z axis that rises 0.5 a turn, from −140° to +140°; cut from the
//! block (the rectangle 8 ≤ x ≤ 12, |z| ≤ 1 revolved from 0° to 60°).
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, revolve_profile_brep, sweep_solid, BooleanOperation, BrepSolid,
    OutlinePiece, ScrewStep, SolidMotion, SolidTool,
};
#[allow(unused_imports)]
use std::f64::consts::{FRAC_PI_2, PI, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let section = [p(8.0, 0.0, -1.0), p(12.0, 0.0, -1.0), p(12.0, 0.0, 1.0), p(8.0, 0.0, 1.0)];
    let block = revolve_profile_brep(&polygon(&section)?, p(0.0, 0.0, 0.0), z, 60f64.to_radians())?;
    let tool = SolidTool::Revolved {
        axis_origin: p(10.0, 0.0, 0.0),
        axis_direction: z,
        radial: p(1.0, 0.0, 0.0),
        meridian: vec![
            OutlinePiece::Line { from: [0.0, -0.5], to: [1.0, -0.5] },
            OutlinePiece::Line { from: [1.0, -0.5], to: [1.0, 0.5] },
            OutlinePiece::Line { from: [1.0, 0.5], to: [0.0, 0.5] },
            OutlinePiece::Line { from: [0.0, 0.5], to: [0.0, -0.5] },
        ],
    };
    let motion = SolidMotion {
        steps: vec![ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: z, turn: 1.0, advance: 0.5 / TAU }],
        from: (-140.0f64).to_radians(),
        to: 140.0f64.to_radians(),
    };
    let mut operands = vec![block];
    operands.push(sweep_solid(&tool, &motion)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
