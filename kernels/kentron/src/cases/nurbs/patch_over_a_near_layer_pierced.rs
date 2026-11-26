//! A block on the floor z = 0 under a face trimmed on a Bézier patch that folds back past the face
//! to a second layer a few ten-thousandths beneath it, with a square hole cut straight through:
//! the box from (2/5, 3/10, −1) to (7/10, 3/5, 3). The patch, degree 4 in u and 1 in v, has the
//! control points (xᵢ, j, zᵢ + j/10) with x = 1, 0, −1/3, 0, 1 and
//! z = 799/1000, 401/2000, 4/5, 399/2000, 801/1000. The top is the face on its parameters
//! 13/20 ≤ u ≤ 1, 0 ≤ v ≤ 1.
use crate::bench::p;
use crate::shared::nurbs::block_under_face;
use kentron::boolean;
use kentron::geometry::surface::{BezierSurface, Surface};
use kentron::make;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let x = [1.0, 0.0, -1.0 / 3.0, 0.0, 1.0];
    let z = [0.799, 0.2005, 0.8, 0.1995, 0.801];
    let poles = (0..5).map(|i| (0..2).map(|j| p(x[i], j as f64, z[i] + j as f64 / 10.0)).collect()).collect();
    let patch = Surface::Bezier(BezierSurface::new(poles));
    let top = make::face_on_domain(m, patch, 0.65, 1.0, 0.0, 1.0)?;
    let block = block_under_face(m, top, 0.0, 1.0)?;
    let hole = Cuboid::from_corners(p(0.4, 0.3, -1.0), p(0.7, 0.6, 3.0)).build(m)?;
    boolean::difference(m, block, hole)
}
