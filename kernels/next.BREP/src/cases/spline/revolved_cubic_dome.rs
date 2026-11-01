//! A dome turned a full turn about the z axis from the region of the xz plane bounded by the z
//! axis from (0, 0, 0) to (0, 0, 10), the x axis from (0, 0, 0) to (9, 0, 0), and the cubic Bézier
//! curve through the control points (9, 0, 0), (12, 0, 4), (9, 0, 10) and (0, 0, 10).
use crate::solids::p;
use crate::solids::sweep::bezier;
use crate::Refusal;
use brep_kernel::{make_line, revolve_profile_brep, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let (origin, rim, top) = (p(0.0, 0.0, 0.0), p(9.0, 0.0, 0.0), p(0.0, 0.0, 10.0));
    let region = vec![
        make_line(origin, rim)?,
        bezier(&[rim, p(12.0, 0.0, 4.0), p(9.0, 0.0, 10.0), top])?,
        make_line(top, origin)?,
    ];
    Ok(revolve_profile_brep(&region, origin, p(0.0, 0.0, 1.0), TAU)?)
}
