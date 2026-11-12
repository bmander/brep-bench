//! The rod of radius 5 about the z axis from z = 0 to z = 20, less everything a wheel of two beads
//! passes through as it slides without turning 40 up the z axis. The beads are tori on one axis
//! parallel to y through (11.5, 0, −9): the first centred there, of radii 6 and 2, the second
//! centred 1.5 further along the axis, of radii 6 and 1.5; the wheel is their union.
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): what a union sweeps is the union of
//! what its parts sweep: each bead, a solid of revolution about the line through (11.5, 0, −9)
//! along y whose meridian is its tube's circle, slid 40 up, and both cut from the rod.
use crate::solids::p;
use crate::solids::swept::{less_all, revolved_tool, slide_by, Piece};
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, sweep_solid, BrepSolid};
use std::f64::consts::{TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), z, 5.0, 20.0)?;
    let bead = |along: f64, tube: f64| revolved_tool(p(11.5, 0.0, -9.0), p(0.0, 1.0, 0.0), x, &[
        Piece::Arc { centre: [6.0, along], radius: tube, start: 0.0, end: TAU },
    ]);
    let up = slide_by(p(0.0, 0.0, 40.0));
    less_all(rod, vec![sweep_solid(&bead(0.0, 2.0), &up)?, sweep_solid(&bead(1.5, 1.5), &up)?])
}
