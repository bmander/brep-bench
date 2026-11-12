//! The ball of radius 5 centred on the origin hollowed to a wall 1 thick with no opening: the ball
//! less the ball of radius 4, its inner sphere facing the void.
//!
//! The kernel's shell with no opening (`offset_shell_closed`), its wall inward: the result is the
//! solid's own shell and, inside it, the cavity's.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_sphere_brep, offset_shell_closed, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let solid = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    Ok(offset_shell_closed(&solid, 1.0)?.solid)
}
