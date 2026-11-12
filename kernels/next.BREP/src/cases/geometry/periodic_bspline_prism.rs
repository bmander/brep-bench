//! A prism from z = 0 to z = 10 on the periodic uniform quadratic B-spline whose control polygon is
//! the square of corners (10, 10), (−10, 10), (−10, −10) and (10, −10).
//!
//! The section is the kernel's periodic B-spline (`make_periodic_bspline`) of degree 2 on those
//! four control points, one closed curve, as the profile of its extrude.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, make_periodic_bspline, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let corners =
        [p(10.0, 10.0, 0.0), p(-10.0, 10.0, 0.0), p(-10.0, -10.0, 0.0), p(10.0, -10.0, 0.0)];
    let section = make_periodic_bspline(2, &corners)?;
    Ok(extrude_profile_brep(&[section], p(0.0, 0.0, 1.0), 10.0)?)
}
