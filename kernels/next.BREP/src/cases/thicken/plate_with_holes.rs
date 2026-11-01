//! The sheet of the square in the xy plane from (−10, −10, 0) to (10, 10, 0) with two round holes
//! of radius 3 about (−5, 0, 0) and (5, 0, 0), thickened by 1 to each side: the plate from z = −1
//! to 1 holed right through by two cylinders.
//!
//! The sheet is the plane's face trimmed in its parameters, which are lengths from (−10, −10, 0):
//! the square counter-clockwise, the two circles clockwise.
use crate::solids::offset::thicken_both;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{make_arc, make_plane, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, y) = (p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0));
    let sheet = make_plane(p(-10.0, -10.0, 0.0), x, y, 20.0, 20.0)?;
    let square =
        polygon(&[p(0.0, 0.0, 0.0), p(20.0, 0.0, 0.0), p(20.0, 20.0, 0.0), p(0.0, 20.0, 0.0)])?;
    let hole = |u: f64| make_arc(p(u, 10.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, -1.0, 0.0), 3.0, 0.0, TAU);
    let loops = [square, vec![hole(5.0)?], vec![hole(15.0)?]];
    thicken_both(&sheet, &loops, 2.0)
}
