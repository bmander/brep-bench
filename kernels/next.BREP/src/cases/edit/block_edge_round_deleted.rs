//! The box from the origin to (20, 10, 10), the edge where its top meets its side x = 20 rounded to
//! radius 2; then the round deleted, the top and the side run on to meet in that edge again: the
//! box.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal` deletes the faces.
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, fillet_edges, make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 10.0, 10.0)?;
    let rounded = fillet_edges(&block, &[p(20.0, 5.0, 10.0)], None, 2.0, false, None)?;
    // the round at its middle: 2 from its axis, the line x = 18, z = 8, at 45°
    let s = 2f64.sqrt();
    let round = face_at(&rounded, p(18.0 + s, 5.0, 8.0 + s))?;
    Ok(delete_faces_and_heal(&rounded, &[round])?)
}
