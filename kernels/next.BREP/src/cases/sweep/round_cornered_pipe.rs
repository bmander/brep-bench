//! A pipe of radius 3 along two straight runs, from (−30, 0, 0) to the origin and on to
//! (0, 30, 0), the section carried around the corner by turning it about the z axis through it.
//!
//! The kernel's sweep with round corners takes the section and the path as they are described:
//! the disk's circle square to the first run, and the three points of the path.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_arc, sweep_profile_round_corners, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let section = make_arc(p(-30.0, 0.0, 0.0), p(0.0, 1.0, 0.0), p(0.0, 0.0, 1.0), 3.0, 0.0, std::f64::consts::TAU)?;
    let path = [p(-30.0, 0.0, 0.0), p(0.0, 0.0, 0.0), p(0.0, 30.0, 0.0)];
    Ok(sweep_profile_round_corners(&[section], &path)?)
}
