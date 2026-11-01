//! A prism standing on the xy plane, from z = 0 to z = 10, whose section is the peanut inside a
//! closed, uniform, periodic cubic B-spline of twelve control points.
//!
//! next.BREP's curves are clamped, so the periodic spline is made by the kernel's exact closed
//! cubic interpolation (`interpolate_curve_closed`), which solves for the periodic cubic on
//! uniform knots and re-expresses it clamped by knot insertion: given the curve's points at its
//! twelve knots, (Pₖ + 4Pₖ₊₁ + Pₖ₊₂)/6 (the description's span formula at t = 0), on the
//! parameters 0, 1, …, 12, the periodic cubic through them is that spline, C² at the seam too. A
//! profile is a loop of at least two curves, so it is split in two at its halfway parameter.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, interpolate_curve_closed, BrepSolid, Vec3};

/// The control points, counter-clockwise round the peanut.
const POLES: [(f64, f64); 12] = [
    (14.0, 0.0), (12.0, 8.0), (4.0, 6.0), (0.0, -21.0 / 8.0), (-4.0, 6.0), (-12.0, 8.0),
    (-14.0, 0.0), (-12.0, -8.0), (-4.0, -6.0), (0.0, 21.0 / 8.0), (4.0, -6.0), (12.0, -8.0),
];

pub fn build() -> Result<BrepSolid, Refusal> {
    let pole = |k: usize| {
        let (x, y) = POLES[k % 12];
        p(x, y, 0.0)
    };
    // the span from Pₖ, Pₖ₊₁, Pₖ₊₂, Pₖ₊₃ starts at (Pₖ + 4Pₖ₊₁ + Pₖ₊₂)/6
    let knots_points: Vec<Vec3> = (0..12)
        .map(|k| pole(k).add(pole(k + 1).scale(4.0)).add(pole(k + 2)).scale(1.0 / 6.0))
        .collect();
    let parameters: Vec<f64> = (0..=12).map(f64::from).collect();
    let peanut = interpolate_curve_closed(&knots_points, &parameters)?;
    let (first, second) = peanut.split(6.0)?;
    Ok(extrude_profile_brep(&[first, second], p(0.0, 0.0, 1.0), 10.0)?)
}
