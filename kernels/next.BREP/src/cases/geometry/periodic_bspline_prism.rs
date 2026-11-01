//! A prism from z = 0 to z = 10 on the periodic uniform quadratic B-spline whose control polygon is
//! the square of corners (10, 10), (−10, 10), (−10, −10) and (10, −10).
//!
//! next.BREP's B-splines are clamped, so the periodic curve is written as what it is: four
//! parabolic arcs, each from the middle of one side over a corner to the middle of the next, as
//! clamped quadratics with a double knot at each side's middle. A profile has at least two curves,
//! so the closed curve is cut in two at (10, 0) and (−10, 0), two arcs each.
use crate::solids::geometry::rounded_square;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let corners =
        [p(10.0, 10.0, 0.0), p(-10.0, 10.0, 0.0), p(-10.0, -10.0, 0.0), p(10.0, -10.0, 0.0)];
    let section = rounded_square(&corners)?;
    Ok(extrude_profile_brep(&section, p(0.0, 0.0, 1.0), 10.0)?)
}
