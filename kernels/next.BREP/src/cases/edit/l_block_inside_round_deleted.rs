//! The L outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15) extruded 8 along +z,
//! its inside corner, the edge x = 5, y = 5, rounded to radius 2; then the round deleted, the faces
//! x = 5 and y = 5 run back to meet in that edge: the L-block with a sharp inside corner.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal` deletes the faces.
use crate::solids::{p, polygon};
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, extrude_profile_brep, fillet_edges, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let outline = polygon(&[
        p(0.0, 0.0, 0.0), p(20.0, 0.0, 0.0), p(20.0, 5.0, 0.0),
        p(5.0, 5.0, 0.0), p(5.0, 15.0, 0.0), p(0.0, 15.0, 0.0),
    ])?;
    let block = extrude_profile_brep(&outline, p(0.0, 0.0, 1.0), 8.0)?;
    let rounded = fillet_edges(&block, &[p(5.0, 5.0, 4.0)], None, 2.0, false, None)?;
    // the round at its middle: 2 from its axis, the line x = 7, y = 7, at 45°
    let s = 2f64.sqrt();
    let round = face_at(&rounded, p(7.0 - s, 7.0 - s, 4.0))?;
    Ok(delete_faces_and_heal(&rounded, &[round])?)
}
