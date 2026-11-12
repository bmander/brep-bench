//! The rod of radius 5 about the z axis from z = 0 to z = 20, less everything a wheel passes
//! through as it slides without turning along the z axis: a torus with its axis parallel to y,
//! the circle through its tube's centres of radius 6 and its tube of radius 2, its centre moving
//! from (11.5, 0, −9) to (11.5, 0, 29).
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the wheel, a solid of revolution
//! about the line through (11.5, 0, −9) along y whose meridian is the circle of radius 2 about the
//! point 6 from the axis, slid 38 up, and cut from the rod.
use crate::solids::p;
use crate::solids::swept::{less_all, revolved_tool, slide_by, Piece};
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, sweep_solid, BrepSolid};
use std::f64::consts::{TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), z, 5.0, 20.0)?;
    let wheel = revolved_tool(p(11.5, 0.0, -9.0), p(0.0, 1.0, 0.0), x, &[
        Piece::Arc { centre: [6.0, 0.0], radius: 2.0, start: 0.0, end: TAU },
    ]);
    less_all(rod, vec![sweep_solid(&wheel, &slide_by(p(0.0, 0.0, 38.0)))?])
}
