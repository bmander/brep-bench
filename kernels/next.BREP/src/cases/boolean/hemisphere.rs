//! A ball of radius 5 about the origin, with everything below the xy plane cut away by a block
//! reaching past it.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    let below = make_box_brep(p(-10.0, -10.0, -10.0), 20.0, 20.0, 10.0)?;
    Ok(cut(&ball, &below)?)
}
