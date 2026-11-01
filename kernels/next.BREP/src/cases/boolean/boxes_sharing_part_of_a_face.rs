//! A cube of side 10 from the origin to (10, 10, 10), joined by a second from (5, 0, 10) to
//! (15, 10, 20) standing on it, its bottom covering half of the first's top.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let first = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 10.0, 10.0)?;
    let second = make_box_brep(p(5.0, 0.0, 10.0), 10.0, 10.0, 10.0)?;
    Ok(union(&first, &second)?)
}
