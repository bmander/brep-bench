//! A ring about the z axis: the periodic uniform quadratic B-spline of the xz plane whose control
//! polygon is the square of corners (13, 0, 3), (7, 0, 3), (7, 0, −3) and (13, 0, −3), turned a
//! whole turn about the z axis.
//!
//! The section is the kernel's periodic B-spline (`make_periodic_bspline`) of degree 2 on those
//! four control points, one closed curve, as the profile of its revolve.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_periodic_bspline, revolve_profile_brep, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let corners = [p(13.0, 0.0, 3.0), p(7.0, 0.0, 3.0), p(7.0, 0.0, -3.0), p(13.0, 0.0, -3.0)];
    let section = make_periodic_bspline(2, &corners)?;
    Ok(revolve_profile_brep(&[section], p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)?)
}
