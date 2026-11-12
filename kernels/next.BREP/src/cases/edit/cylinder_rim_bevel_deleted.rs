//! The cylinder of radius 6 on the z axis from z = 0 to z = 8 with a cone frustum on it, from
//! radius 6 at z = 8 to radius 4 at z = 10: a cylinder with its top rim beveled. Then the bevel,
//! the frustum's side, deleted, the top and the side run on to meet: the cylinder of radius 6 from
//! z = 0 to z = 10.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal` deletes the faces.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, make_cone_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let cylinder = make_cylinder_brep(p(0.0, 0.0, 0.0), z, 6.0, 8.0)?;
    let frustum = make_cone_brep(p(0.0, 0.0, 8.0), z, 6.0, 4.0, 2.0)?;
    let beveled = union(&cylinder, &frustum)?;
    let bevel = face_at(&beveled, p(0.0, -5.0, 9.0))?;
    Ok(delete_faces_and_heal(&beveled, &[bevel])?)
}
