//! A block (1/1000 ≤ x, y ≤ 1, from the floor) under the bicubic patch S(u, v) = (s³, t³, 1/2 + st),
//! s = 2u − 1, t = 2v − 1 (singular at u = v = 1/2 alone), trimmed to 11/20 ≤ u, v ≤ 1, then cut
//! down to −x + y + z/10 ≤ 13/250, a plane passing 0.002 from the patch's singular point.
//!
//! The patch is cadrs's `NurbsSurface` from the case's control points; the block is a rectangle
//! extruded whose top is given the patch over that part of its parameters (Replace face); the
//! part beyond the plane is cut away with a block on it.
use crate::bench::{p3, v3};
use crate::shared::nurbs::{spline_surface, trimmed_block};
use crate::shared::singular::cut_beyond;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let (xs, ss) = ([-1.0, 1.0, -1.0, 1.0], [-1.0, -1.0 / 3.0, 1.0 / 3.0, 1.0]);
    let points: Vec<Vec<[f64; 3]>> =
        (0..4).map(|i| (0..4).map(|j| [xs[i], xs[j], 0.5 + ss[i] * ss[j]]).collect()).collect();
    let bezier = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let patch = spline_surface(3, 3, bezier.clone(), bezier, &points, None)?;
    let block = trimmed_block(k, (0.001, 0.001), (1.0, 1.0), 1.0, patch, ((0.55, 1.0), (0.55, 1.0)))?;
    // (A point of the plane: −x + y + z/10 = 13/250 at x = z = 1/2.)
    cut_beyond(k, block, p3(0.5, 0.502, 0.5), v3(-1.0, 1.0, 0.1))
}
