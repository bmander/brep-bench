//! The solid inside the hyperboloid of one sheet x² + y² ≤ 16(1 + z²/36), −6 ≤ z ≤ 6: the branch
//! x > 0 of the hyperbola x²/16 − z²/36 = 1 of the xz plane turned a whole turn about the z axis,
//! with a flat disc at each end.
use crate::solids::p;
use brep_kernel::{make_hyperbola, make_line, revolve_profile_brep, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, String> {
    // P(t) = (4 cosh t, 6 sinh t): z = ±6 at t = ±arsinh 1, where x = 4√2
    let t = 1.0f64.asinh();
    let r = 4.0 * t.cosh();
    let section = [
        make_hyperbola(p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 4.0, 6.0, -t, t)?,
        make_line(p(r, 0.0, 6.0), p(0.0, 0.0, 6.0))?,
        make_line(p(0.0, 0.0, 6.0), p(0.0, 0.0, -6.0))?,
        make_line(p(0.0, 0.0, -6.0), p(r, 0.0, -6.0))?,
    ];
    revolve_profile_brep(&section, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)
}
