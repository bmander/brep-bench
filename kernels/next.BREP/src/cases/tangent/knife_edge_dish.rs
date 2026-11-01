//! A cylinder of radius 5 on the z axis from z = 0 to 10, less the ball of radius 5 about
//! (0, 0, 10), the middle of its top face: the dish meets the wall tangentially in a knife edge.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let drum = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 10.0)?;
    let ball = make_sphere_brep(p(0.0, 0.0, 10.0), 5.0, p(0.0, 0.0, 1.0))?;
    Ok(cut(&drum, &ball)?)
}
