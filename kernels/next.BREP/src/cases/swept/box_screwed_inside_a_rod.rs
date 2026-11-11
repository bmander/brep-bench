//! A rod with a sealed helical tunnel: everything a small box passes through along a screw inside
//! it.
//!
//! Through the kernel's sweep of a solid in parts (`sweep_solid_parts`): the box 2.5 ≤ x ≤ 3.5,
//! |y| ≤ 0.5, 2 ≤ z ≤ 3 as a convex polyhedron, along the screw about the z axis that rises 4 a
//! turn, from 0° to 720°. Its outer face grazes along a line inside it, past which the sweep would
//! fold over itself, so the kernel sweeps the box's halves either side of the plane through the
//! axis and that line; the rod is cut by both.
use crate::solids::p;
use crate::Refusal;
#[allow(unused_imports)]
use brep_kernel::{
    boolean_operation_nary, make_box_brep, make_cylinder_brep, revolve_profile_brep,
    sweep_solid_parts, BooleanOperation, BrepSolid, OutlinePiece, ScrewStep, SolidMotion,
    SolidTool, Vec3,
};
#[allow(unused_imports)]
use std::f64::consts::{FRAC_PI_2, PI, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let mut operands = vec![rod];
    let tool0 = SolidTool::Polyhedron {
        vertices: (0..8)
            .map(|i| p(if i & 1 != 0 { 3.5 } else { 2.5 }, if i & 2 != 0 { 0.5 } else { -0.5 }, if i & 4 != 0 { 3.0 } else { 2.0 }))
            .collect(),
        faces: vec![vec![0, 2, 3, 1], vec![4, 5, 7, 6], vec![0, 1, 5, 4], vec![2, 6, 7, 3], vec![0, 4, 6, 2], vec![1, 3, 7, 5]],
    };
    let motion0 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 4.0 / TAU },
        ],
        from: 0.0,
        to: (4.0 * PI),
    };
    operands.extend(sweep_solid_parts(&tool0, &motion0)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
