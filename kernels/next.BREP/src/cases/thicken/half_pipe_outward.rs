//! The sheet of the half of the cylinder of radius 5 about the z axis where y ≥ 0, from z = 0 to
//! z = 10, thickened by 1 away from the axis: the half of the tube between radii 5 and 6.
//!
//! The sheet is the line at (5, 0) along z turned half a turn about the z axis, through +y.
use crate::solids::offset::thicken;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_line, make_revolution, BrepSolid};
use std::f64::consts::PI;

pub fn build() -> Result<BrepSolid, Refusal> {
    let line = make_line(p(5.0, 0.0, 0.0), p(5.0, 0.0, 10.0))?;
    let sheet = make_revolution(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), &line, PI)?;
    thicken(&sheet, &[], 1.0, |q| p(q.x, q.y, 0.0))
}
