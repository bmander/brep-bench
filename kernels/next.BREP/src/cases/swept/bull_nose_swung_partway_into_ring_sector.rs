//! The ring sector 8 ≤ ρ ≤ 12, −1 ≤ z ≤ 1, 0° to 60° about the z axis, less everything a bull-nose
//! mill swung into it passes through as it turns rigidly about the z axis from −140° to +30°: the
//! solid of revolution about the upright line through (10, 0): a flat bottom of radius 0.7 at z =
//! −0.5, a quarter torus of tube radius 0.3 about the circle of radius 0.7 at z = −0.2, a side of
//! radius 1 up to a flat top at z = 0.5.
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the mill, a solid of revolution
//! about the upright line through (10, 0, 0) with the meridian described, turned about the z axis
//! from −140° to +30°, its end positions included, and cut from the block.
use crate::solids::p;
use crate::solids::swept::{less_all, revolved_tool, ring_sector, turn_about_z, Piece};
use crate::Refusal;
use brep_kernel::{sweep_solid, BrepSolid};
use std::f64::consts::{FRAC_PI_2};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let mill = revolved_tool(p(10.0, 0.0, 0.0), z, x, &[
        Piece::Line { from: [0.0, -0.5], to: [0.7, -0.5] },
        Piece::Arc { centre: [0.7, -0.2], radius: 0.3, start: -FRAC_PI_2, end: 0.0 },
        Piece::Line { from: [1.0, -0.2], to: [1.0, 0.5] },
        Piece::Line { from: [1.0, 0.5], to: [0.0, 0.5] },
        Piece::Line { from: [0.0, 0.5], to: [0.0, -0.5] },
    ]);
    less_all(ring_sector(8.0, 12.0, -1.0, 1.0, 0.0, 60f64.to_radians())?, vec![sweep_solid(&mill, &turn_about_z((-140f64).to_radians(), 30f64.to_radians(), 0.0))?])
}
