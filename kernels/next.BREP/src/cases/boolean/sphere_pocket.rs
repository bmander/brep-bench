//! A block from (−10, −10, −10) to (10, 10, 0), less the ball of radius 5 about the origin: a
//! hemispherical pocket in its top face.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(-10.0, -10.0, -10.0), 20.0, 20.0, 10.0)?;
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    Ok(cut(&block, &ball)?)
}
