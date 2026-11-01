//! A ball of radius 15 about the origin, united with a solid cone on the z axis whose apex is at
//! (0, 0, −25) and whose base is the disc of radius 12 at z = −9: the cone's side is tangent to
//! the sphere along its base's rim.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cone_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 15.0, p(0.0, 0.0, 1.0))?;
    // From its base at z = −9 down the −z axis to its apex, 16 below.
    let cone = make_cone_brep(p(0.0, 0.0, -9.0), p(0.0, 0.0, -1.0), 12.0, 0.0, 16.0)?;
    Ok(union(&ball, &cone)?)
}
