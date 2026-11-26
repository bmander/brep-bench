//! A pipe of radius 1 along the helix of radius 10 about the z axis that rises 5 a turn, starting
//! at (10, 0, 0) and turning right-handed for forty whole turns, square to the path, its ends
//! flat: the kernel's helical sweep, as `sweep/helical-pipe` makes its three turns.
//!
//! `sweep_profile_helix` starts its helix at angle zero from the axis's own perpendicular, which
//! for the z axis is +y, so the pipe is turned a quarter turn back about z to start on +x.
use crate::solids::p;
use crate::solids::sweep::{circle, moved, turn_z};
use crate::Refusal;
use brep_kernel::{sweep_profile_helix, BrepSolid};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    // the section is placed on the helix by the sweep; only its radius matters
    let section = circle(p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), 1.0)?;
    let coil = sweep_profile_helix(&section, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 10.0, 5.0, 40.0,
        None)?;
    Ok(moved(&coil, turn_z(-FRAC_PI_2, p(0.0, 0.0, 0.0))?)?)
}
