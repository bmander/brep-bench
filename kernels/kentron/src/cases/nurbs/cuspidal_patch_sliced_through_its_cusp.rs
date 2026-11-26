//! A block on the floor z = 0 under a face trimmed on a Bézier patch with a cusp a hundredth past
//! the face, cut by the plane z = 11y/10, which passes through the cusp. The patch, degree 3 in u
//! and 1 in v, has the control points (xᵢ, j, zᵢ + j/10) with x = 1, −1/3, −1/3, 1 and
//! z = −1/2, 3/2, −1/2, 3/2; its cusp runs along u = 1/2. The top is the face on its parameters
//! 11/20 ≤ u ≤ 1, 0 ≤ v ≤ 1. What is cut away is the half-space above the plane.
use crate::bench::p;
use crate::shared::boolean::half_space;
use crate::shared::nurbs::block_under_face;
use kentron::boolean;
use kentron::geometry::surface::{BezierSurface, Surface};
use kentron::make;
use kentron::math::Direction;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, crate::Refusal> {
    let x = [1.0, -1.0 / 3.0, -1.0 / 3.0, 1.0];
    let z = [-0.5, 1.5, -0.5, 1.5];
    let poles = (0..4).map(|i| (0..2).map(|j| p(x[i], j as f64, z[i] + j as f64 / 10.0)).collect()).collect();
    let patch = Surface::Bezier(BezierSurface::new(poles));
    let top = make::face_on_domain(m, patch, 0.55, 1.0, 0.0, 1.0)?;
    let block = block_under_face(m, top, 0.0, 1.0)?;
    let above = half_space(m, p(0.0, 0.0, 0.0), Direction::new(0.0, -1.1, 1.0), p(0.0, 0.0, 50.0))?;
    Ok(boolean::difference(m, block, above)?)
}
