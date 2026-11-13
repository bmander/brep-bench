//! Everything a rod passes through as it turns rigidly about the z axis by 120°, right-handed: the
//! solid cylinder of radius 0.5 about the x axis from x = −3 to x = 5, its ends flat.
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the rod, a solid of revolution
//! about the x axis whose meridian is the rectangle out to radius 0.5 from x = −3 to x = 5, turned
//! 120° about the z axis.
use crate::solids::p;
use crate::solids::swept::{revolved_tool, turn_about_z, Piece};
use crate::Refusal;
use brep_kernel::{sweep_solid, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let rod = revolved_tool(p(-3.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0), &[
        Piece::Line { from: [0.0, 0.0], to: [0.5, 0.0] },
        Piece::Line { from: [0.5, 0.0], to: [0.5, 8.0] },
        Piece::Line { from: [0.5, 8.0], to: [0.0, 8.0] },
        Piece::Line { from: [0.0, 8.0], to: [0.0, 0.0] },
    ]);
    Ok(sweep_solid(&rod, &turn_about_z(0.0, 120f64.to_radians(), 0.0))?)
}
