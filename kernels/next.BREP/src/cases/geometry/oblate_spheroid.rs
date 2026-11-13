//! The oblate spheroid (x² + y²)/36 + z²/9 ≤ 1: the ellipse x²/36 + z²/9 ≤ 1 of the xz plane turned
//! about the z axis. Its half with x ≥ 0, the kernel's ellipse arc (`make_ellipse_arc`, semi-axes 6
//! along x and 3 along z) from its lowest point to its highest and the segment of the axis between
//! them, is revolved a whole turn.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_ellipse_arc, make_line, revolve_profile_brep, BrepSolid};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (o, x, z) = (p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let half = [
        make_ellipse_arc(o, x, z, 6.0, 3.0, -FRAC_PI_2, FRAC_PI_2)?,
        make_line(p(0.0, 0.0, 3.0), p(0.0, 0.0, -3.0))?,
    ];
    Ok(revolve_profile_brep(&half, o, z, TAU)?)
}
