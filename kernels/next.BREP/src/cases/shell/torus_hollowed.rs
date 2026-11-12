//! The solid torus about the z axis whose tube has radius 2 about a middle circle of radius 6,
//! hollowed to a wall 0.5 thick with no opening: the torus less the torus of tube radius 1.5, its
//! inner torus facing the void.
//!
//! The kernel's shell with no opening (`offset_shell_closed`), its wall inward: the result is the
//! solid's own shell and, inside it, the cavity's.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_torus_brep, offset_shell_closed, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let solid = make_torus_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 6.0, 2.0)?;
    Ok(offset_shell_closed(&solid, 0.5)?.solid)
}
