//! A cube of side 10 from (−5, −5, −5) to (5, 5, 5), and the ball about the origin of radius
//! 5√2, tangent to its twelve edges at their midpoints: their common part.
use crate::solids::boolean::common;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cube = make_box_brep(p(-5.0, -5.0, -5.0), 10.0, 10.0, 10.0)?;
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0 * 2f64.sqrt(), p(0.0, 0.0, 1.0))?;
    Ok(common(&cube, &ball)?)
}
