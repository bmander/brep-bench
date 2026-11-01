//! A ball's cap turned a full turn about the z axis from the region of the xz plane bounded by the
//! z axis from (0, 0, −5) to (0, 0, 10), the segment from (0, 0, −5) to (5√3, 0, −5), and the
//! rational quadratic Bézier arc from (5√3, 0, −5) over (10√3, 0, 10) to (0, 0, 10), weights 1, ½
//! and 1: the arc of the circle of radius 10 about the origin.
use crate::solids::p;
use crate::solids::sweep::rational_bezier;
use crate::Refusal;
use brep_kernel::{make_line, revolve_profile_brep, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let r3 = 3f64.sqrt();
    let (foot, rim, top) = (p(0.0, 0.0, -5.0), p(5.0 * r3, 0.0, -5.0), p(0.0, 0.0, 10.0));
    let region = vec![
        make_line(foot, rim)?,
        rational_bezier(&[rim, p(10.0 * r3, 0.0, 10.0), top], &[1.0, 0.5, 1.0])?,
        make_line(top, foot)?,
    ];
    Ok(revolve_profile_brep(&region, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)?)
}
