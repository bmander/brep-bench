//! The box from the origin to (20, 16, 12) with every edge rounded to radius 2: twelve quarter
//! cylinders and eight eighths of a ball. Then all twenty deleted, the six flat faces run on to
//! meet in the edges and the corners again: the box.
//!
//! The box is a rectangle extruded and all its edges filleted together, radius 2; then Delete face
//! takes the twenty rounds.
use crate::bench::{block, p3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let (a, b, c) = (20.0, 16.0, 12.0);
    let bx = block(k, p3(0.0, 0.0, 0.0), p3(a, b, c))?;
    let edges = k.edges_but_seams(bx)?;
    let rounded = fillet(k, bx, &edges, 2.0)?;
    // A point of each round at 45° about its axis, and of each ball along the diagonal from its
    // center: `near(u, len)` is that far in from the face at 0 (u = 0) or at `len` (u = 1).
    let near = |u: f64, len: f64, d: f64| if u == 0.0 { 2.0 - d } else { len - 2.0 + d };
    let (s, t) = (2f64.sqrt(), 2.0 / 3f64.sqrt());
    let mut blends = Vec::new();
    for u in [0.0, 1.0] {
        for v in [0.0, 1.0] {
            blends.push(p3(near(u, a, s), near(v, b, s), c / 2.0));
            blends.push(p3(a / 2.0, near(u, b, s), near(v, c, s)));
            blends.push(p3(near(u, a, s), b / 2.0, near(v, c, s)));
            for w in [0.0, 1.0] {
                blends.push(p3(near(u, a, t), near(v, b, t), near(w, c, t)));
            }
        }
    }
    let faces = k.faces_at(rounded, &blends)?;
    deleted(k, rounded, &faces)
}
