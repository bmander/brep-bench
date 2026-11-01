//! A stepped shaft on the z axis, the section in the xz plane turned a full turn about z: radius
//! 20 from z = 0 to z = 60 and radius 12 from z = 60 to z = 160, the inside corner where the
//! shoulder meets the thin part the quarter circle of radius 4 about (16, 64).
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{make_arc, revolve_profile_brep, BrepSolid};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    // The section in the plane y = 0: (x, z) as (x, 0, z).
    let xz = |x: f64, z: f64| p(x, 0.0, z);
    let mut section = polygon(&[xz(0.0, 0.0), xz(20.0, 0.0), xz(20.0, 60.0), xz(16.0, 60.0)])?;
    section.pop(); // the polygon's closing side; the section goes on round the arc
    // From (16, 60) to (12, 64) about (16, 64), bulging toward the corner (12, 60): angle 0 along
    // −z, a quarter turn along −x.
    let (minus_z, minus_x) = (p(0.0, 0.0, -1.0), p(-1.0, 0.0, 0.0));
    section.push(make_arc(xz(16.0, 64.0), minus_z, minus_x, 4.0, 0.0, FRAC_PI_2)?);
    let mut rest = polygon(&[xz(12.0, 64.0), xz(12.0, 160.0), xz(0.0, 160.0), xz(0.0, 0.0)])?;
    rest.pop();
    section.extend(rest);
    Ok(revolve_profile_brep(&section, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)?)
}
