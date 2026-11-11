//! The solid a thick disc fills as it is turned 90° about the z axis, which passes through its rim.
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the disc is a solid of revolution
//! about the line through (2, 0, 0) along y, its meridian the rectangle out to radius 2 from y = 0
//! to y = 0.5; turned about the z axis from 0° to 90°.
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
    let mut bodies = Vec::new();
    let tool0 = SolidTool::Revolved {
        axis_origin: p(2.0, 0.0, 0.0),
        axis_direction: p(0.0, 1.0, 0.0),
        radial: p(1.0, 0.0, 0.0),
        meridian: vec![
            OutlinePiece::Line { from: [0.0, 0.0], to: [2.0, 0.0] },
            OutlinePiece::Line { from: [2.0, 0.0], to: [2.0, 0.5] },
            OutlinePiece::Line { from: [2.0, 0.5], to: [0.0, 0.5] },
            OutlinePiece::Line { from: [0.0, 0.5], to: [0.0, 0.0] },
        ],
    };
    let motion0 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 0.0 },
        ],
        from: 0.0,
        to: FRAC_PI_2,
    };
    bodies.push(sweep_solid(&tool0, &motion0)?);
    Ok(boolean_operation_nary(&bodies, BooleanOperation::Union)?)
}
