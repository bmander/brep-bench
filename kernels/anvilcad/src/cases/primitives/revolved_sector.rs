//! The rectangle 6 ≤ x ≤ 10, 0 ≤ z ≤ 4 of the xz plane turned 90° about the z axis, from +x
//! toward +y: a quarter of a washer.
use std::f64::consts::FRAC_PI_2;

use anvil_brep::{Solid, revolve};
use glam::DVec2;

use crate::shared::{e, rect, region, xz};

pub fn build() -> Result<Solid, String> {
    // the sketch's y axis is the world's z axis; a positive angle turns the profile toward the
    // sketch plane's normal, which is −y, so a quarter turn toward +y is a negative one
    let section = region(&[rect((6., 0.), (10., 4.))])?;
    revolve::revolve(&xz([0., 0., 0.]), &section, DVec2::ZERO, DVec2::Y, -FRAC_PI_2).map_err(e)
}
