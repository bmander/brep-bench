//! A napkin ring, the ball of radius 5 about the origin with a hole of radius 3 bored through it
//! along the z axis, grown by 1 to its parallel body: the sphere of radius 6 outside, the bore
//! narrowed to radius 2, and each rim a piece of a torus's tube of radius 1.
//!
//! The hole is cut by a cylinder running past both ends of the ball.
use crate::solids::offset::{parallel, subtract};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    let bore = make_cylinder_brep(p(0.0, 0.0, -6.0), p(0.0, 0.0, 1.0), 3.0, 12.0)?;
    let ring = subtract(&ball, &bore)?;
    Ok(parallel(&ring, 1.0)?)
}
