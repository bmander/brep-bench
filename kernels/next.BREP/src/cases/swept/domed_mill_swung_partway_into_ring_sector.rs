//! The ring sector 8 ≤ ρ ≤ 12, −1 ≤ z ≤ 1, 0° to 60° about the z axis, less everything a domed mill
//! swung into it passes through as it turns rigidly about the z axis from −140° to +30°: the solid
//! of revolution about the upright line through (10, 0): a cylinder of radius 1 up to z = 3, ended
//! below by the cap of the sphere of radius 2 about (10, 0, 1.2).
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the mill, a solid of revolution
//! about the upright line through (10, 0, 0) whose meridian is the sphere's arc from its lowest
//! point out to radius 1, the side up to z = 3 and the flat top, turned about the z axis from −140°
//! to +30°, its end positions included, and cut from the block.
use crate::solids::p;
use crate::solids::swept::{less_all, revolved_tool, ring_sector, turn_about_z, Piece};
use crate::Refusal;
use brep_kernel::{sweep_solid, BrepSolid};
use std::f64::consts::{FRAC_PI_2, PI};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let shoulder = 1.2 - 3f64.sqrt();
    let mill = revolved_tool(p(10.0, 0.0, 0.0), z, x, &[
        Piece::Arc { centre: [0.0, 1.2], radius: 2.0, start: -FRAC_PI_2, end: -PI / 3.0 },
        Piece::Line { from: [1.0, shoulder], to: [1.0, 3.0] },
        Piece::Line { from: [1.0, 3.0], to: [0.0, 3.0] },
        Piece::Line { from: [0.0, 3.0], to: [0.0, -0.8] },
    ]);
    less_all(ring_sector(8.0, 12.0, -1.0, 1.0, 0.0, 60f64.to_radians())?, vec![sweep_solid(&mill, &turn_about_z((-140f64).to_radians(), 30f64.to_radians(), 0.0))?])
}
