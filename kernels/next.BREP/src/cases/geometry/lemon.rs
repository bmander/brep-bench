//! A lemon: the segment of the disc of radius 5 about (−3, 0, 0) of the xz plane with x ≥ 0,
//! between its arc through (2, 0, 0) and its chord on the z axis from (0, 0, −4) to (0, 0, 4),
//! turned a whole turn about the z axis.
use crate::solids::p;
use brep_kernel::{make_arc, make_line, revolve_profile_brep, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, String> {
    // the arc meets the axis where cos φ = 3/5 about its centre
    let theta = (3.0f64 / 5.0).acos();
    let segment = [
        make_arc(p(-3.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, -theta, theta)?,
        make_line(p(0.0, 0.0, 4.0), p(0.0, 0.0, -4.0))?,
    ];
    revolve_profile_brep(&segment, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)
}
