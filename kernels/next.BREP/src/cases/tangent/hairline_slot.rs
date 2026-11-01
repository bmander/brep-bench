//! A cube of side 10 from the origin to (10, 10, 10), with a saw cut a millionth wide down from
//! the top: the block 5 ≤ x ≤ 5.000001, z ≥ 5, cut by a block that runs past the cube's front,
//! back and top.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cube = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 10.0, 10.0)?;
    let saw = make_box_brep(p(5.0, -1.0, 5.0), 0.000001, 12.0, 6.0)?;
    Ok(cut(&cube, &saw)?)
}
