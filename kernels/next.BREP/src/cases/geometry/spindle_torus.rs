//! A spindle torus about the z axis, middle circle radius 3 and tube radius 5, counted once: the
//! part x ≥ 0 of the disc (x − 3)² + z² ≤ 25 of the xz plane (the tube's outer arc, from
//! (0, 0, −4) through (8, 0, 0) to (0, 0, 4), and its chord on the axis) turned a whole turn.
use crate::solids::p;
use brep_kernel::{make_arc, make_line, revolve_profile_brep, BrepSolid};
use std::f64::consts::{PI, TAU};

pub fn build() -> Result<BrepSolid, String> {
    // the arc meets the axis where cos φ = −3/5 about its centre
    let phi = PI - (3.0f64 / 5.0).acos();
    let apple = [
        make_arc(p(3.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, -phi, phi)?,
        make_line(p(0.0, 0.0, 4.0), p(0.0, 0.0, -4.0))?,
    ];
    revolve_profile_brep(&apple, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)
}
