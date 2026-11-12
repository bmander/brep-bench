//! The solid a ring fills as it slides 6 along x, square to its axis, less than its hole's width:
//! the torus about the z axis centred on the origin, the circle through its tube's centres of
//! radius 6, its tube of radius 2.
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the ring, a solid of revolution
//! about the z axis whose meridian is the circle of radius 2 about the point 6 from the axis, slid
//! 6 along x.
use crate::solids::p;
use crate::solids::swept::{revolved_tool, slide_by, Piece};
use crate::Refusal;
use brep_kernel::{sweep_solid, BrepSolid};
use std::f64::consts::{TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let ring = revolved_tool(p(0.0, 0.0, 0.0), z, x, &[
        Piece::Arc { centre: [6.0, 0.0], radius: 2.0, start: 0.0, end: TAU },
    ]);
    Ok(sweep_solid(&ring, &slide_by(p(6.0, 0.0, 0.0)))?)
}
