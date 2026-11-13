//! The box from the origin to (10, 8, 6) shelled outward to a wall 1 thick, open at its top, the
//! wall every point outside the box within 1 of its other faces: the box grown by 1 and cut off at
//! z = 6, less the box. Its outside is rounded, quarter cylinders about the edges and eighths of a
//! ball at the bottom corners.
//!
//! The kernel's shell joined by arcs (`offset_shell_rounded`), outward, open at the top, picked by
//! a point inside it.
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{make_box_brep, offset_shell_rounded, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 8.0, 6.0)?;
    let top = face_at(&block, p(5.0, 4.0, 6.0))?;
    Ok(offset_shell_rounded(&block, &[top], -1.0)?)
}
