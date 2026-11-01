//! A cube of side 10 from the origin to (10, 10, 10) and its inscribed ball, of radius 5 about
//! (5, 5, 5): their common part, the whole ball.
use crate::solids::boolean::common;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cube = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 10.0, 10.0)?;
    let ball = make_sphere_brep(p(5.0, 5.0, 5.0), 5.0, p(0.0, 0.0, 1.0))?;
    Ok(common(&cube, &ball)?)
}
