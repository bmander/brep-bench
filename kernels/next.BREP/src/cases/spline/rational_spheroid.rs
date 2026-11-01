//! A spheroid turned a full turn about the z axis from the region of the xz plane between the z
//! axis from (0, 0, −10) to (0, 0, 10) and a rational quadratic B-spline joining them: five control
//! points (0, 0, −10), (6, 0, −10), (6, 0, 0), (6, 0, 10), (0, 0, 10), weights 1, √2/2, 1, √2/2, 1,
//! on the clamped knots (0, 0, 0, ½, ½, 1, 1, 1), two quarters of the ellipse x²/36 + z²/100 = 1.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_line, revolve_profile_brep, BrepSolid, NurbsCurve, Vec4};
use std::f64::consts::{FRAC_1_SQRT_2, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let poles = [(0.0, -10.0, 1.0), (6.0, -10.0, FRAC_1_SQRT_2), (6.0, 0.0, 1.0),
        (6.0, 10.0, FRAC_1_SQRT_2), (0.0, 10.0, 1.0)];
    let meridian = NurbsCurve::new(2, vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0],
        poles.iter().map(|&(x, z, w)| Vec4::from_point(p(x, 0.0, z), w)).collect())?;
    let region = vec![meridian, make_line(p(0.0, 0.0, 10.0), p(0.0, 0.0, -10.0))?];
    Ok(revolve_profile_brep(&region, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)?)
}
