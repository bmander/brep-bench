//! Two spherical sectors of a ball of radius 10 about the origin, each the part of the ball
//! within 45° of its axis: the first about +z, the second the first turned 60° about the y axis.
//! The solid common to both.
//!
//! A sector is a solid of revolution: its section in the xz plane, the line from the apex out
//! along the cone's ruling to the sphere at 45° from the axis, the arc of the sphere up to the
//! axis, and the axis back down to the apex, turned a whole turn about the z axis.
use crate::bench::{arc, common, line, rotated};
use crate::Refusal;
use solidcore_ops::sweep::revolve;
use solidcore_ops::Body;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, TAU};

/// The sector of the ball of radius `r` about +z.
fn sector(r: f64) -> Result<Body, Refusal> {
    let rim = [r * FRAC_PI_4.cos(), 0.0, r * FRAC_PI_4.sin()];
    let section = [
        line([0.0, 0.0, 0.0], rim),
        // about -y, angle 0 along +x, so counter-clockwise runs from +x up towards +z
        arc([0.0; 3], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0], r, FRAC_PI_4, FRAC_PI_2),
        line([0.0, 0.0, r], [0.0, 0.0, 0.0]),
    ];
    Ok(revolve(&section, [0.0; 3], [0.0, 0.0, 1.0], TAU, true)?)
}

pub fn build() -> Result<Body, Refusal> {
    shared_apex(10.0)
}

/// The two sectors of the ball of radius `r`, and their common part.
pub fn shared_apex(r: f64) -> Result<Body, Refusal> {
    let first = sector(r)?;
    let second = rotated(sector(r)?, [0.0; 3], [0.0, 1.0, 0.0], 60.0)?;
    common(first, second)
}
