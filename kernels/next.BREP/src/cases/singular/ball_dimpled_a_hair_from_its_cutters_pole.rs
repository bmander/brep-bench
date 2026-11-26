//! The ball of radius 10 about the origin less the ball of radius 0.8 about (ρ cos 120°,
//! ρ sin 120°, 5.501), ρ = √(10.4² − 5.501²), each with its poles on the vertical through its
//! center: the circle where they meet passes a hair from the small ball's lower pole.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let height = 5.501;
    let rho = (10.4f64 * 10.4 - height * height).sqrt();
    let a = 120f64.to_radians();
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 10.0, p(0.0, 0.0, 1.0))?;
    let cutter = make_sphere_brep(p(rho * a.cos(), rho * a.sin(), height), 0.8, p(0.0, 0.0, 1.0))?;
    Ok(cut(&ball, &cutter)?)
}
