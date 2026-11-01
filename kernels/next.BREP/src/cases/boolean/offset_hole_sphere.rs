//! A ball of radius 20 about the origin, bored through parallel to the y axis by a hole of radius
//! 6 whose axis is the line x = 5, z = 0: the drill a cylinder from y = −21 to y = 21.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 20.0, p(0.0, 0.0, 1.0))?;
    let drill = make_cylinder_brep(p(5.0, -21.0, 0.0), p(0.0, 1.0, 0.0), 6.0, 42.0)?;
    Ok(cut(&ball, &drill)?)
}
