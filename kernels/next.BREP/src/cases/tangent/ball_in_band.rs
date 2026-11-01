//! A ball of radius 5 about the origin, united with a cylinder of the same radius on the z axis
//! from z = −3 to z = 3, whose wall touches the ball along its equator: the band with a spherical
//! cap on each end.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    let band = make_cylinder_brep(p(0.0, 0.0, -3.0), p(0.0, 0.0, 1.0), 5.0, 6.0)?;
    Ok(union(&ball, &band)?)
}
