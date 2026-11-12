//! The box from the origin to (10, 8, 6) hollowed to a wall 1 thick with no opening: the box less
//! the box from (1, 1, 1) to (9, 7, 5), its cavity's shell facing the void.
//!
//! The kernel's shell with no opening (`offset_shell_closed`), its wall inward: the result is the
//! solid's own shell and, inside it, the cavity's.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, offset_shell_closed, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let solid = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 8.0, 6.0)?;
    Ok(offset_shell_closed(&solid, 1.0)?.solid)
}
