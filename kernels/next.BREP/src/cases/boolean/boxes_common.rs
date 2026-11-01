//! Two blocks 10 high on the xy plane, spanning x and y from 0 to 10 and from 5 to 15: their
//! common part.
use crate::solids::boolean::common;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let first = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 10.0, 10.0)?;
    let second = make_box_brep(p(5.0, 5.0, 0.0), 10.0, 10.0, 10.0)?;
    Ok(common(&first, &second)?)
}
