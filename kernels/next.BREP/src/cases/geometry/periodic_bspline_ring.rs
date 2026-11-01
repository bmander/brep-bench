//! A ring about the z axis: the periodic uniform quadratic B-spline of the xz plane whose control
//! polygon is the square of corners (13, 0, 3), (7, 0, 3), (7, 0, −3) and (13, 0, −3), turned a
//! whole turn about the z axis.
//!
//! next.BREP's B-splines are clamped, so the periodic curve is written as what it is: four
//! parabolic arcs, each from the middle of one side over a corner to the middle of the next, as
//! clamped quadratics with a double knot at each side's middle, cut in two (a profile has at least
//! two curves) at (13, 0, 0) and (7, 0, 0).
use crate::solids::geometry::rounded_square;
use crate::solids::p;
use brep_kernel::{revolve_profile_brep, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, String> {
    let corners = [p(13.0, 0.0, 3.0), p(7.0, 0.0, 3.0), p(7.0, 0.0, -3.0), p(13.0, 0.0, -3.0)];
    let section = rounded_square(&corners)?;
    revolve_profile_brep(&section, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)
}
