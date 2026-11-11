//! A block bored into by a ring leaning 35° pushed straight down, stopping inside it.
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the ring is a solid of revolution
//! about the line through (0, 0, 4) along (sin 35°, 0, cos 35°), its meridian the circle of radius 1
//! at 3 from that axis; slid 8 down the z axis; cut from the block.
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
        axis_direction: p(35f64.to_radians().sin(), 0.0, 35f64.to_radians().cos()),
        radial: p(35f64.to_radians().cos(), 0.0, -(35f64.to_radians().sin())),
        meridian: vec![OutlinePiece::Arc { centre: [3.0, 0.0], radius: 1.0, start: 0.0, end: TAU }],
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
