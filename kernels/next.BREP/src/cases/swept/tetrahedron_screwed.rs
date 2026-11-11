//! The solid a regular tetrahedron fills as it is carried along a screw.
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the tetrahedron as a convex
//! polyhedron, its four corners and four faces, along the screw about the z axis that turns 1 and
//! rises 5/2π per radian, from 0 to 3π (540°). The kernel bounds the sweep by the tetrahedron's back
//! faces at the start, its front faces at the end, and the sheets its silhouette edges sweep between.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{sweep_solid, BrepSolid, ScrewStep, SolidMotion, SolidTool};
use std::f64::consts::{PI, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let tool = SolidTool::Polyhedron {
        vertices: vec![p(5.0, 1.0, 1.0), p(5.0, -1.0, -1.0), p(3.0, 1.0, -1.0), p(3.0, -1.0, 1.0)],
        faces: vec![vec![0, 1, 2], vec![0, 1, 3], vec![0, 2, 3], vec![1, 2, 3]],
    };
    let motion = SolidMotion {
        steps: vec![ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 5.0 / TAU }],
        from: 0.0,
        to: 3.0 * PI,
    };
    Ok(sweep_solid(&tool, &motion)?)
}
