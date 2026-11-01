//! The box from the origin to (10, 8, 6) shelled inward to a wall 1 thick, open at every face but
//! its bottom: only the bottom is a wall, and the result is the slab from the origin to
//! (10, 8, 1).
use crate::solids::p;
use crate::solids::shell::shell;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 8.0, 6.0)?;
    let openings = [
        p(5.0, 4.0, 6.0), p(5.0, 0.0, 3.0), p(5.0, 8.0, 3.0), p(0.0, 4.0, 3.0), p(10.0, 4.0, 3.0),
    ];
    shell(&block, &openings, 1.0)
}
