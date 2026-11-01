//! Two cubes of side 10 side by side, from the origin to (10, 10, 10) and from (10, 0, 0) to
//! (20, 10, 10), joined: the face they share at x = 10 goes.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let first = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 10.0, 10.0)?;
    let second = make_box_brep(p(10.0, 0.0, 0.0), 10.0, 10.0, 10.0)?;
    Ok(union(&first, &second)?)
}
