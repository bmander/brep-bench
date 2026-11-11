//! A block bored into by a bull-nose mill leaning 30° pushed straight down, stopping inside it.
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the mill is a solid of revolution
//! about the line through (0, 0, 4) along (sin 30°, 0, cos 30°), its meridian the flat end out to
//! 1.5, the quarter circle of radius 0.5 about (1.5, 0.5), the side at 2 up to 3 and the flat top;
//! slid 8 down the z axis; cut from the block.
use crate::solids::p;
use crate::Refusal;
#[allow(unused_imports)]
use brep_kernel::{
    boolean_operation_nary, make_box_brep, make_cylinder_brep, revolve_profile_brep,
    sweep_solid, BooleanOperation, BrepSolid, OutlinePiece, ScrewStep, SolidMotion,
    SolidTool, Vec3,
};
#[allow(unused_imports)]
use std::f64::consts::{FRAC_PI_2, PI, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(-8.0, -8.0, -8.0), 16.0, 16.0, 8.0)?;
    let mut operands = vec![block];
    let tool0 = SolidTool::Revolved {
        axis_origin: p(0.0, 0.0, 4.0),
        axis_direction: p(30f64.to_radians().sin(), 0.0, 30f64.to_radians().cos()),
        radial: p(30f64.to_radians().cos(), 0.0, -(30f64.to_radians().sin())),
        meridian: vec![
            OutlinePiece::Line { from: [0.0, 0.0], to: [1.5, 0.0] },
            OutlinePiece::Arc { centre: [1.5, 0.5], radius: 0.5, start: -FRAC_PI_2, end: 0.0 },
            OutlinePiece::Line { from: [2.0, 0.5], to: [2.0, 3.0] },
            OutlinePiece::Line { from: [2.0, 3.0], to: [0.0, 3.0] },
            OutlinePiece::Line { from: [0.0, 3.0], to: [0.0, 0.0] },
        ],
    };
    let motion0 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 0.0, advance: -8.0 },
        ],
        from: 0.0,
        to: 1.0,
    };
    operands.push(sweep_solid(&tool0, &motion0)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
