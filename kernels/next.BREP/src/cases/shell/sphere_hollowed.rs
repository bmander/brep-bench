//! The ball of radius 5 centred on the origin hollowed to a wall 1 thick with no opening: the ball
//! less the ball of radius 4, its inner sphere facing the void.
//!
//! The kernel's `offset_shell` needs at least one opening face. With none, the hollow is the
//! kernel's own shell recipe (`offset_shell_by_boolean`): the solid less its offset body, its
//! sphere moved 1 inward by `offset_solid`.
use crate::solids::p;
use crate::solids::shell::less;
use crate::Refusal;
use brep_kernel::{make_sphere_brep, offset_solid, BrepSolid, OffsetCorners};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    let cavity = offset_solid(&ball, -1.0, OffsetCorners::Sharp)?.solid;
    less(&ball, &cavity)
}
