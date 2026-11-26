//! A block on the floor z = 0 under a face trimmed on a Bézier patch that folds back over itself
//! past the face, with a corner notched out by the box from (1/20, −1, −1) to (1/2, 1/2, 3). The
//! patch, degree 4 in u and 1 in v, has the control points (xᵢ, j, zᵢ + j/10) with
//! x = 1, 0, −1/3, 0, 1 and z = 4/5, 1/5, 4/5, 1/5, 4/5; it folds along u = 1/2. The top is the
//! face on its parameters 13/20 ≤ u ≤ 1, 0 ≤ v ≤ 1.
use crate::bench::p;
use crate::shared::nurbs::block_under_face;
use kentron::boolean;
use kentron::geometry::surface::{BezierSurface, Surface};
use kentron::make;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let x = [1.0, 0.0, -1.0 / 3.0, 0.0, 1.0];
    let z = [0.8, 0.2, 0.8, 0.2, 0.8];
    let poles = (0..5).map(|i| (0..2).map(|j| p(x[i], j as f64, z[i] + j as f64 / 10.0)).collect()).collect();
    let patch = Surface::Bezier(BezierSurface::new(poles));
    let top = make::face_on_domain(m, patch, 0.65, 1.0, 0.0, 1.0)?;
    let block = block_under_face(m, top, 0.0, 1.0)?;
    let notch = Cuboid::from_corners(p(0.05, -1.0, -1.0), p(0.5, 0.5, 3.0)).build(m)?;
    boolean::difference(m, block, notch)
}
