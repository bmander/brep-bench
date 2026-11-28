//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a sealed helical
//! tunnel: what the box 2.5 ≤ x ≤ 3.5, −0.5 ≤ y ≤ 0.5, 2 ≤ z ≤ 3 passes through as the screw about
//! the z axis that rises 4 a turn carries it from a turn of 0° to one of 720°.
//!
//! solidcraft-kernel has no sweep of a solid, so the box's sweep is built by elementary geometry
//! from the kernel's helical sweep of planar regions (`sweep_helix`). On each cylinder about the
//! axis the screw slides the cylinder along itself, and unrolled, the box's trace there is one
//! rectangle or two, each from z = 2 to z = 3 between two angles. A rectangle slid along a line
//! passes over itself at each end and over what is swept by any path across it from its top edge
//! down to its bottom edge, joined to both edges. Here those are the box's faces z = 2 and z = 3,
//! and its sections by half-planes through the axis: the one at angle 0 crosses the box's trace
//! out to radius 3.5, and those through the box's outer upright edges, at ±atan(0.5/3.5), cross
//! each of the two rectangles beyond it. Every one of them crosses the screw's motion, never
//! moving along itself, so its screw sweep is what it passes through. With the box where it
//! starts and where it ends, these are the tunnel.
use super::square_bar_screwed_through_rod::{screw, screwed};
use crate::solids::*;

pub fn build() -> R<Body> {
    rod(1.0)
}

/// The rod and its tunnel with every length `s` times as long.
pub fn rod(s: f64) -> R<Body> {
    placed_rod(s, &|piece| Ok(piece))
}

/// The rod and its tunnel with every length `s` times as long, each piece (the rod, the box at
/// either end, each sweep) moved by `place` as it is made, before the cuts.
pub fn placed_rod(s: f64, place: &dyn Fn(Body) -> R<Body>) -> R<Body> {
    let (lead, start, stop) = (4.0 * s, 0.0, 720.0);
    let mut tunnel = vec![
        place(block((2.5 * s, -0.5 * s, 2.0 * s), (3.5 * s, 0.5 * s, 3.0 * s))?)?,
        place(block((2.5 * s, -0.5 * s, 10.0 * s), (3.5 * s, 0.5 * s, 11.0 * s))?)?,
    ];
    for z in [2.0 * s, 3.0 * s] {
        let level = Plane { origin: v3(0.0, 0.0, z), x: Vec3::X, y: Vec3::Y };
        tunnel.push(place(screw(&level, &rect(2.5 * s, -0.5 * s, 3.5 * s, 0.5 * s), lead, start, stop)?)?);
    }
    tunnel.push(place(screw(&xz(), &rect(2.5 * s, 2.0 * s, 3.5 * s, 3.0 * s), lead, start, stop)?)?);
    // the half-planes through the corner edges (3.5, ±0.5), from the face x = 2.5 to the corner
    let corner = ((3.5 * s) * (3.5 * s) + (0.5 * s) * (0.5 * s)).sqrt();
    let angle = (0.5f64 / 3.5).atan().to_degrees();
    for side in [-1.0, 1.0] {
        let half = screwed(&xz(), 0.0, side * angle);
        let inner = 2.5 * s * corner / (3.5 * s);
        tunnel.push(place(screw(&half, &rect(inner, 2.0 * s, corner, 3.0 * s), lead, start, stop)?)?);
    }
    let mut rod = place(zcyl(0.0, 0.0, 5.0 * s, 0.0, 20.0 * s)?)?;
    for piece in &tunnel {
        rod = cut(&rod, piece)?;
    }
    Ok(rod)
}
