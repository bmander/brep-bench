//! The rectangle 6 ≤ x ≤ 10, 0 ≤ z ≤ 4 of the xz plane turned a full turn about the z axis.
use std::f64::consts::TAU;

use anvil_brep::{Solid, revolve};
use glam::DVec2;

use crate::shared::{e, rect, region, xz};

pub fn build() -> Result<Solid, String> {
    // the sketch's y axis is the world's z axis
    let section = region(&[rect((6., 0.), (10., 4.))])?;
    revolve::revolve(&xz([0., 0., 0.]), &section, DVec2::ZERO, DVec2::Y, TAU).map_err(e)
}
