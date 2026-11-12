//! The ring sector 8 ≤ ρ ≤ 12, −1 ≤ z ≤ 1, 0° to 60° about the z axis, less everything a drill
//! swung into it passes through as it turns rigidly about the z axis from −140° to +30°: the solid
//! of revolution about the upright line through (10, 0): a cylinder of radius 1 from z = −0.8 +
//! 1/tan 59° to 3, ended below by a cone of 118° with its point at (10, 0, −0.8).
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the drill, a solid of revolution
//! about the upright line through (10, 0, 0) whose meridian is the cone's side from its point, the
//! cylinder's side and the flat top, turned about the z axis from −140° to +30°, its end positions
//! included, and cut from the block.
use crate::solids::p;
use crate::solids::swept::{less_all, revolved_tool, ring_sector, turn_about_z, Piece};
use crate::Refusal;
use brep_kernel::{sweep_solid, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let shoulder = -0.8 + 1.0 / 59f64.to_radians().tan();
    let drill = revolved_tool(p(10.0, 0.0, 0.0), z, x, &[
        Piece::Line { from: [0.0, -0.8], to: [1.0, shoulder] },
        Piece::Line { from: [1.0, shoulder], to: [1.0, 3.0] },
        Piece::Line { from: [1.0, 3.0], to: [0.0, 3.0] },
        Piece::Line { from: [0.0, 3.0], to: [0.0, -0.8] },
    ]);
    less_all(ring_sector(8.0, 12.0, -1.0, 1.0, 0.0, 60f64.to_radians())?, vec![sweep_solid(&drill, &turn_about_z((-140f64).to_radians(), 30f64.to_radians(), 0.0))?])
}
