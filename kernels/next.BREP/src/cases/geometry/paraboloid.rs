//! The solid paraboloid x² + y² ≤ 8z, z ≤ 8: the parabola x² = 8z of the xz plane (focal length 2,
//! from its vertex to x = 8) turned a whole turn about the z axis, closed by the disc at z = 8.
use crate::solids::p;
use brep_kernel::{make_line, make_parabola, revolve_profile_brep, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, String> {
    // x² = 4·2·z, P(t) = (2t², 4t) in the frame (axis +z, latus +x): t = 2 reaches x = 8, z = 8
    let section = [
        make_parabola(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), p(1.0, 0.0, 0.0), 2.0, 0.0, 2.0)?,
        make_line(p(8.0, 0.0, 8.0), p(0.0, 0.0, 8.0))?,
        make_line(p(0.0, 0.0, 8.0), p(0.0, 0.0, 0.0))?,
    ];
    revolve_profile_brep(&section, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)
}
