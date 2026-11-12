//! The pentagon through (0, 0), (20, 0), (20, 7), (17, 10) and (0, 10) in the plane y = 0, corners
//! as (x, z), extruded 10 along +y: a block with one edge beveled. Then the bevel deleted, the top
//! and the side run on to meet: the box from the origin to (20, 10, 10).
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal` deletes the faces.
use crate::solids::{p, polygon};
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, extrude_profile_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let section = polygon(&[
        p(0.0, 0.0, 0.0), p(20.0, 0.0, 0.0), p(20.0, 0.0, 7.0),
        p(17.0, 0.0, 10.0), p(0.0, 0.0, 10.0),
    ])?;
    let beveled = extrude_profile_brep(&section, p(0.0, 1.0, 0.0), 10.0)?;
    let bevel = face_at(&beveled, p(18.5, 5.0, 8.5))?;
    Ok(delete_faces_and_heal(&beveled, &[bevel])?)
}
