//! The solid inside the upper sheet of z²/9 − (x² + y²)/16 = 1, below z = 6: the branch z > 0 of
//! the hyperbola z²/9 − x²/16 = 1 of the xz plane, from its vertex (0, 0, 3) to x = 4√3, turned a
//! whole turn about the z axis, closed by the disc at z = 6.
use crate::solids::p;
use brep_kernel::{make_hyperbola, make_line, revolve_profile_brep, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, String> {
    // P(t) = (3 cosh t, 4 sinh t) along (z, x): z = 6 at cosh t = 2, where x = 4 sinh t = 4√3
    let t = 2.0f64.acosh();
    let r = 4.0 * t.sinh();
    let section = [
        make_hyperbola(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), p(1.0, 0.0, 0.0), 3.0, 4.0, 0.0, t)?,
        make_line(p(r, 0.0, 6.0), p(0.0, 0.0, 6.0))?,
        make_line(p(0.0, 0.0, 6.0), p(0.0, 0.0, 3.0))?,
    ];
    revolve_profile_brep(&section, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)
}
