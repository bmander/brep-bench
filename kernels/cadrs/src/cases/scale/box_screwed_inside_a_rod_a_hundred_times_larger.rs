//! A rod (radius 500, z from 0 to 2000) with a sealed helical tunnel: everything the box
//! 250 ≤ x ≤ 350, −50 ≤ y ≤ 50, 200 ≤ z ≤ 300 passes through as it is screwed about the z axis,
//! rising 400 a turn, from 0° to 720° (`swept/box-screwed-inside-a-rod` a hundred times larger).
//!
//! The box is a rectangle extruded; its screw is cadrs's sweep of a solid, cut from the rod.
use crate::bench::{block, cut, cylinder, p3, v3};
use crate::shared::swept::{screwed, z_axis};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let tool = block(k, p3(250.0, -50.0, 200.0), p3(350.0, 50.0, 300.0))?;
    let swept = screwed(k, tool, z_axis(), 400.0, 0.0, 720.0)?;
    let r = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 500.0, 2000.0)?;
    cut(k, r, swept)
}
