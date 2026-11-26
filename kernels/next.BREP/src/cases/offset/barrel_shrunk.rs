//! The ball of radius 5 about the origin between the planes z = −3 and z = 3, shrunk by 1 to its
//! inner parallel body.
use crate::solids::boolean::common;
use crate::solids::offset::parallel;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    // the slab between the planes, wider than the ball
    let slab = make_box_brep(p(-6.0, -6.0, -3.0), 12.0, 12.0, 6.0)?;
    Ok(parallel(&common(&ball, &slab)?, -1.0)?)
}
