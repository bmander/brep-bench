//! The solid a square bar (the square with corners (4, 0), (6, 2), (8, 0), (6, −2), from z = 0 to
//! 3) fills as it turns a quarter turn about the z axis.
//!
//! The square sketched and extruded; what it passes through is cadrs's sweep of a solid (one
//! turn).
use crate::bench::extrude;
use crate::shared::swept::{swung, z_axis};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let square = Loop::polygon(&[[4.0, 0.0], [6.0, 2.0], [8.0, 0.0], [6.0, -2.0]]);
    let bar = extrude(k, &Profile::of(Plane::xy(0.0), vec![square]), 3.0)?;
    swung(k, bar, z_axis(), 0.0, 90.0)
}
