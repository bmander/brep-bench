//! A block on the floor z = 0 under a face trimmed on a bicubic Bézier patch that pinches to a
//! point just outside the face, cut by the plane −x + y + z/10 = 13/250, which passes 0.002 from
//! the pinch. The patch has the control points (xᵢ, xⱼ, 1/2 + sᵢsⱼ) with x = −1, 1, −1, 1 and
//! s = −1, −1/3, 1/3, 1; its parametrization is singular at u = v = 1/2 alone. The top is the face
//! on its parameters 11/20 ≤ u, v ≤ 1. What is cut away is the half-space where
//! −x + y + z/10 > 13/250.
use crate::bench::p;
use crate::shared::boolean::half_space;
use crate::shared::nurbs::block_under_face;
use kentron::boolean;
use kentron::geometry::surface::{BezierSurface, Surface};
use kentron::make;
use kentron::math::Direction;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, crate::Refusal> {
    let x = [-1.0, 1.0, -1.0, 1.0];
    let s = [-1.0, -1.0 / 3.0, 1.0 / 3.0, 1.0];
    let poles = (0..4).map(|i| (0..4).map(|j| p(x[i], x[j], 0.5 + s[i] * s[j])).collect()).collect();
    let patch = Surface::Bezier(BezierSurface::new(poles));
    let top = make::face_on_domain(m, patch, 0.55, 1.0, 0.55, 1.0)?;
    let block = block_under_face(m, top, 0.0, 1.0)?;
    // the plane through (0, 0, 13/25), its normal (−1, 1, 1/10) toward the side cut away
    let beyond = half_space(m, p(0.0, 0.0, 0.52), Direction::new(-1.0, 1.0, 0.1), p(-10.0, 10.0, 0.0))?;
    Ok(boolean::difference(m, block, beyond)?)
}
