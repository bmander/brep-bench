//! A ball of radius 5 about the origin, bored through along the z axis by a hole of radius 3: the
//! drill a cylinder from z = −6 to z = 6, past the ball at both ends.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    let drill = make_cylinder_brep(p(0.0, 0.0, -6.0), p(0.0, 0.0, 1.0), 3.0, 12.0)?;
    Ok(cut(&ball, &drill)?)
}
