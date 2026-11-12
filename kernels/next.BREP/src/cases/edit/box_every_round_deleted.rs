//! The box from the origin to (20, 16, 12) with every edge rounded to radius 2: twelve quarter
//! cylinders and eight eighths of a ball. Then all twenty deleted, the six flat faces run on to
//! meet in the edges and the corners again: the box.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal` deletes the faces.
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, fillet_edges, make_box_brep, BrepSolid, Vec3};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (a, b, c) = (20.0, 16.0, 12.0);
    let block = make_box_brep(p(0.0, 0.0, 0.0), a, b, c)?;
    // A point of each round at 45° about its axis, and of each ball along the diagonal from its
    // center: `near(u, len)` is that far in from the face at 0 (u = 0) or at `len` (u = 1).
    let near = |u: f64, len: f64, d: f64| if u == 0.0 { 2.0 - d } else { len - 2.0 + d };
    let (s, t) = (2f64.sqrt(), 2.0 / 3f64.sqrt());
    let (mut edges, mut blends): (Vec<Vec3>, Vec<Vec3>) = (Vec::new(), Vec::new());
    for u in [0.0, 1.0] {
        for v in [0.0, 1.0] {
            edges.push(p(u * a, v * b, c / 2.0));
            edges.push(p(a / 2.0, u * b, v * c));
            edges.push(p(u * a, b / 2.0, v * c));
            blends.push(p(near(u, a, s), near(v, b, s), c / 2.0));
            blends.push(p(a / 2.0, near(u, b, s), near(v, c, s)));
            blends.push(p(near(u, a, s), b / 2.0, near(v, c, s)));
            for w in [0.0, 1.0] {
                blends.push(p(near(u, a, t), near(v, b, t), near(w, c, t)));
            }
        }
    }
    let rounded = fillet_edges(&block, &edges, None, 2.0, false, None)?;
    let faces = blends.iter().map(|&q| face_at(&rounded, q)).collect::<Result<Vec<_>, _>>()?;
    Ok(delete_faces_and_heal(&rounded, &faces)?)
}
