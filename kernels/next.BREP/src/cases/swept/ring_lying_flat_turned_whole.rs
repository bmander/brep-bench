//! Everything a ring passes through as it turns a whole turn about the z axis: the torus about the
//! upright line through (8, 0), the circle through its tube's centres of radius 3 in z = 0, its
//! tube of radius 1.
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the ring, a solid of revolution
//! about the upright line through (8, 0, 0) whose meridian is the circle of radius 1 about the
//! point 3 from the axis, turned a whole turn about the z axis.
use crate::solids::p;
use crate::solids::swept::{revolved_tool, turn_about_z, Piece};
use crate::Refusal;
use brep_kernel::{sweep_solid, BrepSolid};
use std::f64::consts::{TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let ring = revolved_tool(p(8.0, 0.0, 0.0), z, x, &[
        Piece::Arc { centre: [3.0, 0.0], radius: 1.0, start: 0.0, end: TAU },
    ]);
    Ok(sweep_solid(&ring, &turn_about_z(0.0, TAU, 0.0))?)
}
