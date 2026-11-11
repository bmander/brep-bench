//! A ring with a sealed channel cut inside it by a leaning drum turned 100° about its axis.
//!
//! The kernel's whole sweep of a solid (`sweep_solid`): the drum is a solid of revolution about the
//! line through (5, 1.2, 0) along (sin 25°, 0, cos 25°), its meridian the rectangle out to radius 1
//! from 1.5 below its middle to 1.5 above; turned about the z axis from 0° to 100°; cut from the ring
//! (the cylinder of radius 8 less that of radius 3, from z = −2.5 to 2.5).
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
    let outer = make_cylinder_brep(p(0.0, 0.0, -2.5), p(0.0, 0.0, 1.0), 8.0, 5.0)?;
    let inner = make_cylinder_brep(p(0.0, 0.0, -3.0), p(0.0, 0.0, 1.0), 3.0, 6.0)?;
    let ring = boolean_operation_nary(&[outer, inner], BooleanOperation::Subtract)?;
    let mut operands = vec![ring];
    let tool0 = SolidTool::Revolved {
        axis_origin: p(5.0, 1.2, 0.0),
        axis_direction: p(25f64.to_radians().sin(), 0.0, 25f64.to_radians().cos()),
        radial: p(0.0, 1.0, 0.0),
        meridian: vec![
            OutlinePiece::Line { from: [0.0, -1.5], to: [1.0, -1.5] },
            OutlinePiece::Line { from: [1.0, -1.5], to: [1.0, 1.5] },
            OutlinePiece::Line { from: [1.0, 1.5], to: [0.0, 1.5] },
            OutlinePiece::Line { from: [0.0, 1.5], to: [0.0, -1.5] },
        ],
    };
    let motion0 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 0.0 },
        ],
        from: 0.0,
        to: 100f64.to_radians(),
    };
    operands.push(sweep_solid(&tool0, &motion0)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
