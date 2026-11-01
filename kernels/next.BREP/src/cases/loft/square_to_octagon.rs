//! A transition from the square −20 ≤ x, y ≤ 20 at z = 0 to the octagon at z = 30 with corners
//! (6, −14), (14, −6), (14, 6), (6, 14), (−6, 14), (−14, 6), (−14, −6), (−6, −14): each side of the
//! square joined to the octagon's side parallel to it by a flat trapezoid, each corner of the
//! square to the octagon's cut side facing it by a flat triangle.
//!
//! A two-section loft rails the k-th curve of one section to the k-th of the other, so the square
//! is given as eight curves too: its four sides, each followed by its corner as a curve of no
//! length, which the loft rails to the cut side facing it.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{loft_profile_brep, make_line, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    // counter-clockwise; the square's side from (−20, −20) to (20, −20) faces the octagon's from
    // (−6, −14) to (6, −14), and its corner (20, −20) the cut side from (6, −14) to (14, −6)
    let square = [(-20.0, -20.0), (20.0, -20.0), (20.0, 20.0), (-20.0, 20.0)];
    let octagon = [(-6.0, -14.0), (6.0, -14.0), (14.0, -6.0), (14.0, 6.0), (6.0, 14.0),
        (-6.0, 14.0), (-14.0, 6.0), (-14.0, -6.0)];
    let at = |(x, y): (f64, f64), z: f64| p(x, y, z);
    let mut foot = Vec::new();
    for k in 0..4 {
        let (from, to) = (at(square[k], 0.0), at(square[(k + 1) % 4], 0.0));
        foot.push(make_line(from, to)?);
        foot.push(make_line(to, to)?);
    }
    let top = (0..8)
        .map(|k| make_line(at(octagon[k], 30.0), at(octagon[(k + 1) % 8], 30.0)))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(loft_profile_brep(&[foot, top])?)
}
