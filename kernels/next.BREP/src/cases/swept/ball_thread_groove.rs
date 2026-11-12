//! The rod of radius 5 about the z axis from z = 0 to z = 20, less everything a ball of radius 1
//! passes through as its centre follows the helix of radius 5 about the z axis from (5, 0, −2),
//! turning right-handed and rising 4 a turn for six turns, to (5, 0, 22).
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the ball, a solid of revolution
//! about the upright line through (5, 0, −2) whose meridian is the half disc of radius 1, carried
//! six turns along the screw about the z axis that rises 4 a turn, and cut from the rod.
use crate::solids::p;
use crate::solids::swept::{less_all, revolved_tool, turn_about_z, Piece};
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, sweep_solid, BrepSolid};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), z, 5.0, 20.0)?;
    let ball = revolved_tool(p(5.0, 0.0, -2.0), z, x, &[
        Piece::Arc { centre: [0.0, 0.0], radius: 1.0, start: -FRAC_PI_2, end: FRAC_PI_2 },
        Piece::Line { from: [0.0, 1.0], to: [0.0, -1.0] },
    ]);
    less_all(rod, vec![sweep_solid(&ball, &turn_about_z(0.0, 6.0 * TAU, 4.0 / TAU))?])
}
