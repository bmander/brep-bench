//! A rod threaded partway up by a bar carried along a screw that stops inside it.
//!
//! Through the kernel's sweep of a solid within a region (`sweep_solid_clipped`): the bar, the box
//! 4 ≤ x ≤ 12, |y| ≤ 8, |z| ≤ 1, as a convex polyhedron, along the screw about the z axis that rises
//! 6 a turn, from −120° to +600°, asked for within 5.2 of the z axis from z = −0.4 to 20.4, which
//! holds the rod (the screw maps that cylinder onto itself, so the kernel may sweep only the bar's
//! part that reaches it); cut from the rod.
use crate::solids::p;
use crate::Refusal;
#[allow(unused_imports)]
use brep_kernel::{
    boolean_operation_nary, make_box_brep, make_cylinder_brep, revolve_profile_brep,
    sweep_solid_clipped, sweep_solid_within, BooleanOperation, BrepSolid, OutlinePiece, ScrewStep, SolidMotion,
    SolidTool, SweepClip, SweptTool, Vec3,
};
#[allow(unused_imports)]
use std::f64::consts::{FRAC_PI_2, PI, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let mut operands = vec![rod];
    let tool0 = SolidTool::Polyhedron {
        vertices: (0..8)
            .map(|i| p(if i & 1 != 0 { 12.0 } else { 4.0 }, if i & 2 != 0 { 8.0 } else { -8.0 }, if i & 4 != 0 { 1.0 } else { -1.0 }))
            .collect(),
        faces: vec![vec![0, 2, 3, 1], vec![4, 5, 7, 6], vec![0, 1, 5, 4], vec![2, 6, 7, 3], vec![0, 4, 6, 2], vec![1, 3, 7, 5]],
    };
    let motion0 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 6.0 / TAU },
        ],
        from: (-120.0f64).to_radians(),
        to: 600.0f64.to_radians(),
    };
    let clip0 = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [0.0, 5.2],
        axial: [-0.4, 20.4],
        angle: None,
    };
    operands.push(sweep_solid_clipped(&tool0, &motion0, &clip0)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
