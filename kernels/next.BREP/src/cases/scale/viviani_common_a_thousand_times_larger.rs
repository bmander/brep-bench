//! `tangent/viviani-common` a thousand times larger: the ball of radius 10000 about the origin, its
//! poles on the y axis and its seam through (−10000, 0, 0), and the cylinder of radius 5000 on the
//! line x = 5000, y = 0 from z = −12000 to z = 12000, its seam on the line x = 0, y = 0: their
//! common, through Viviani's curve. Built as the original is, every length times 1000.
use crate::solids::boolean::common;
use crate::solids::p;
use crate::solids::singular::turned;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, make_sphere_brep_framed, BrepSolid};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep_framed(p(0.0, 0.0, 0.0), 10000.0, p(0.0, 1.0, 0.0),
                                       Some(p(-1.0, 0.0, 0.0)))?;
    // the kernel's seam on a z-axis cylinder runs along +y; a quarter turn about z puts it at −x
    let cylinder = make_cylinder_brep(p(0.0, 0.0, -12000.0), p(0.0, 0.0, 1.0), 5000.0, 24000.0)?;
    let cylinder = turned(&cylinder, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), FRAC_PI_2,
                          p(5000.0, 0.0, 0.0))?;
    Ok(common(&ball, &cylinder)?)
}
