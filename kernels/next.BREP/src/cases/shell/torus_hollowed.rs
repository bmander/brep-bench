//! The solid torus about the z axis whose tube has radius 2 about a middle circle of radius 6,
//! hollowed to a wall 0.5 thick with no opening: the torus less the torus of tube radius 1.5, its
//! inner torus facing the void.
//!
//! The kernel's `offset_shell` needs at least one opening face. With none, the hollow is the
//! kernel's own shell recipe (`offset_shell_by_boolean`): the solid less its offset body, its
//! torus moved 0.5 inward by `offset_solid`.
use crate::solids::p;
use crate::solids::shell::less;
use crate::Refusal;
use brep_kernel::{make_torus_brep, offset_solid, BrepSolid, OffsetCorners};

pub fn build() -> Result<BrepSolid, Refusal> {
    let torus = make_torus_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 6.0, 2.0)?;
    let cavity = offset_solid(&torus, -0.5, OffsetCorners::Sharp)?.solid;
    less(&torus, &cavity)
}
