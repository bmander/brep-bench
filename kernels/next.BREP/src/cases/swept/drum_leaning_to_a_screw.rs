//! A ring sector grooved by a drum leaning to the screw that carries it.
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the drum is a solid of revolution
//! about the line through (10, 0, 0) along (0, sin 25°, cos 25°), its meridian the rectangle out to
//! radius 0.8 from 1.5 below its middle to 1.5 above, swept by the screw about the z axis that
//! rises 1 a turn, from −140° to +140°; cut from the block (the rectangle 8 ≤ x ≤ 12, |z| ≤ 1
//! revolved from 0° to 60°).
use crate::solids::swept::{less_all, revolved_tool, turn_about_z, Piece};
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{revolve_profile_brep, sweep_solid, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let section = [p(8.0, 0.0, -1.0), p(12.0, 0.0, -1.0), p(12.0, 0.0, 1.0), p(8.0, 0.0, 1.0)];
    let block = revolve_profile_brep(&polygon(&section)?, p(0.0, 0.0, 0.0), z, 60f64.to_radians())?;
    let lean = 25f64.to_radians();
    let drum = revolved_tool(p(10.0, 0.0, 0.0), p(0.0, lean.sin(), lean.cos()), p(1.0, 0.0, 0.0), &[
        Piece::Line { from: [0.0, -1.5], to: [0.8, -1.5] },
        Piece::Line { from: [0.8, -1.5], to: [0.8, 1.5] },
        Piece::Line { from: [0.8, 1.5], to: [0.0, 1.5] },
        Piece::Line { from: [0.0, 1.5], to: [0.0, -1.5] },
    ]);
    let groove = sweep_solid(&drum, &turn_about_z((-140f64).to_radians(), 140f64.to_radians(), 1.0 / TAU))?;
    less_all(block, vec![groove])
}
