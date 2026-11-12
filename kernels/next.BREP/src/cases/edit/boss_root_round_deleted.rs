//! The cylinder of radius 8 on the z axis from z = −4 to z = 0 with the cylinder of radius 3 from z
//! = 0 to z = 8 standing on it, the concave edge at its foot rounded to radius 1; then the round, a
//! quarter torus, deleted, the plate's top and the boss's side run on to meet in that edge: the
//! plate and the boss, sharp where they meet.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal` deletes the faces.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, fillet_edges, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let plate = make_cylinder_brep(p(0.0, 0.0, -4.0), z, 8.0, 4.0)?;
    let boss = make_cylinder_brep(p(0.0, 0.0, 0.0), z, 3.0, 8.0)?;
    let foot = p(0.0, -3.0, 0.0);
    let rounded = fillet_edges(&union(&plate, &boss)?, &[foot], None, 1.0, false, None)?;
    // the round at 45° round its tube, whose center line is the circle of radius 4 at z = 1
    let s = 0.5f64.sqrt();
    let round = face_at(&rounded, p(0.0, -(4.0 - s), 1.0 - s))?;
    Ok(delete_faces_and_heal(&rounded, &[round])?)
}
