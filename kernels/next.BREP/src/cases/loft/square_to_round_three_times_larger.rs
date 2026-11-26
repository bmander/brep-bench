//! `loft/square-to-round` three times larger: a transition from the square −60 ≤ x, y ≤ 60 at
//! z = 0 to the circle of radius 45 at z = 90 about the z axis: each side of the square joined by a
//! flat triangle to the point of the circle nearest it, each corner of the square to the quarter
//! circle between those points by a piece of an oblique cone, its apex the corner.
//!
//! Built as the original is, every length times 3: a two-section loft of eight curves each, the
//! square's four sides each followed by its corner as a curve of no length, the circle's four
//! points nearest the sides each followed by the quarter circle to the next.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{loft_profile_brep, make_arc, make_line, BrepSolid};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    // counter-clockwise from the side facing −y, whose nearest point of the circle is (0, −45, 90)
    let square = [(-60.0, -60.0), (60.0, -60.0), (60.0, 60.0), (-60.0, 60.0)];
    let (centre, x, y) = (p(0.0, 0.0, 90.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0));
    let mut foot = Vec::new();
    let mut top = Vec::new();
    for k in 0..4 {
        let (from, to) = (square[k], square[(k + 1) % 4]);
        let corner = p(to.0, to.1, 0.0);
        foot.push(make_line(p(from.0, from.1, 0.0), corner)?);
        foot.push(make_line(corner, corner)?);
        let start = -FRAC_PI_2 + FRAC_PI_2 * k as f64;
        let near = p(45.0 * start.cos(), 45.0 * start.sin(), 90.0);
        top.push(make_line(near, near)?);
        top.push(make_arc(centre, x, y, 45.0, start, start + FRAC_PI_2)?);
    }
    Ok(loft_profile_brep(&[foot, top])?)
}
