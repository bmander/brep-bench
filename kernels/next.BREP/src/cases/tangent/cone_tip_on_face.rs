//! A block from (−10, −10, 0) to (10, 10, 10), united with a solid cone on the z axis whose apex
//! is the origin, the middle of the block's bottom face, and whose base is the disc of radius 5 at
//! z = 15: the tip touches the bottom face from inside at a point.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_cone_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(-10.0, -10.0, 0.0), 20.0, 20.0, 10.0)?;
    // From its base at z = 15 down the −z axis to its apex, 15 below.
    let cone = make_cone_brep(p(0.0, 0.0, 15.0), p(0.0, 0.0, -1.0), 5.0, 0.0, 15.0)?;
    Ok(union(&block, &cone)?)
}
