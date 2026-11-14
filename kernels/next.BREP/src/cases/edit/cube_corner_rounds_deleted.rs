//! The box from the origin to (10, 10, 10), the three edges meeting at (10, 10, 10) rounded to
//! radius 2, an eighth of a ball where they meet; then the three rounds and the ball deleted, the
//! faces x = 10, y = 10 and z = 10 run on to meet in the edges and the corner again: the cube.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal` deletes the faces.
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, fillet_edges, make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cube = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 10.0, 10.0)?;
    let edges = [p(5.0, 10.0, 10.0), p(10.0, 5.0, 10.0), p(10.0, 10.0, 5.0)];
    let rounded = fillet_edges(&cube, &edges, None, 2.0, false, None)?;
    // each round at 45° about its axis, and the ball along the diagonal from its center (8, 8, 8)
    let (s, t) = (8.0 + 2f64.sqrt(), 8.0 + 2.0 / 3f64.sqrt());
    let blends = [p(4.0, s, s), p(s, 4.0, s), p(s, s, 4.0), p(t, t, t)];
    let faces = blends.iter().map(|&q| face_at(&rounded, q)).collect::<Result<Vec<_>, _>>()?;
    Ok(delete_faces_and_heal(&rounded, &faces)?)
}
