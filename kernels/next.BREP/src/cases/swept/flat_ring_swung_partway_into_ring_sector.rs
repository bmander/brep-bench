//! The ring sector 8 ≤ ρ ≤ 12, −1 ≤ z ≤ 1, 0° to 60° about the z axis, less everything a ring lying
//! flat passes through as it turns rigidly about the z axis from −140° to +30°: the torus about the
//! upright line through (10, 0), the circle through its tube's centres of radius 1 in z = 0, its
//! tube of radius 0.4.
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the ring, a solid of revolution
//! about the upright line through (10, 0, 0) whose meridian is the circle of radius 0.4 about the
//! point 1 from the axis, turned about the z axis from −140° to +30°, its end positions included,
//! and cut from the block.
use crate::solids::p;
use crate::solids::swept::{less_all, revolved_tool, ring_sector, turn_about_z, Piece};
use crate::Refusal;
use brep_kernel::{sweep_solid, BrepSolid};
use std::f64::consts::{TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let ring = revolved_tool(p(10.0, 0.0, 0.0), z, x, &[
        Piece::Arc { centre: [1.0, 0.0], radius: 0.4, start: 0.0, end: TAU },
    ]);
    less_all(ring_sector(8.0, 12.0, -1.0, 1.0, 0.0, 60f64.to_radians())?, vec![sweep_solid(&ring, &turn_about_z((-140f64).to_radians(), 30f64.to_radians(), 0.0))?])
}
