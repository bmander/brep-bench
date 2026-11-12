//! The cylinder of radius 6 on the z axis from z = 0 to z = 10, its top rim rounded to radius 2;
//! then the round, a quarter torus, deleted, the top and the side run on to meet in the rim again:
//! the cylinder.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal` deletes the faces.
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, fillet_edges, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cylinder = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 6.0, 10.0)?;
    let rounded = fillet_edges(&cylinder, &[p(0.0, -6.0, 10.0)], None, 2.0, false, None)?;
    // the round at 45° round its tube, whose center line is the circle of radius 4 at z = 8
    let s = 2f64.sqrt();
    let round = face_at(&rounded, p(0.0, -(4.0 + s), 8.0 + s))?;
    Ok(delete_faces_and_heal(&rounded, &[round])?)
}
