//! The box from the origin to (20, 10, 10), drilled through along z by a hole of radius 2 through
//! (10, 5) and countersunk at the top by the cone from radius 2 at z = 8 to radius 4 at z = 10;
//! then the countersink deleted, the hole's wall and the top run on to meet: the block with the
//! plain hole.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal` deletes the faces.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{
    delete_faces_and_heal, make_box_brep, make_cone_brep, make_cylinder_brep, BrepSolid,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let block = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 10.0, 10.0)?;
    let drilled = cut(&block, &make_cylinder_brep(p(10.0, 5.0, -1.0), z, 2.0, 12.0)?)?;
    // the countersink's cone, run on past the top to z = 11, where its radius is 5
    let countersunk = cut(&drilled, &make_cone_brep(p(10.0, 5.0, 8.0), z, 2.0, 5.0, 3.0)?)?;
    let countersink = face_at(&countersunk, p(13.0, 5.0, 9.0))?;
    Ok(delete_faces_and_heal(&countersunk, &[countersink])?)
}
